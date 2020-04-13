use crate::color::Color;
use crate::math::{Point, Vector3};
use crate::scene::{
    material::{Material, SurfaceType, TextureCoords},
    Distance, Scene,
};

pub const SHADOW_BIAS: Distance = 1e-12;
pub const MAX_RECURSION: usize = 25;

use std::f64;

use image::{DynamicImage, ImageBuffer, Rgba};

pub struct Ray {
    pub origin: Point,
    pub direction: Vector3,
}

impl Ray {
    /// 坐标系是z向外，x向右，y向上。是个右手系。
    /// 相机放在z=0处，朝负z方向看；胶片在-1.0处摆放，东西都放到负z那边去
    /// 所以这里的射线的x和y就是从原点出发到胶片的某个像素的中心，z都是-1.0
    /// y这里反一下是因为image的y是朝下的，我们是y朝上
    pub fn new_prime(x: u32, y: u32, scene: &Scene) -> Self {
        assert!(scene.width > scene.height);
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let sensor_x =
            (((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio * fov_adjustment;
        let sensor_y = -(((y as f64 + 0.5) / scene.height as f64) * 2.0 - 1.0) * fov_adjustment;

        Self {
            origin: Point::zero(),
            direction: Vector3 {
                x: sensor_x,
                y: sensor_y,
                z: -1.0,
            }
            .normalize(),
        }
    }

    pub fn create_reflection(
        normal: Vector3,
        incident: Vector3,
        intersection: Point,
        bias: Distance,
    ) -> Self {
        Ray {
            origin: intersection + (normal * bias),
            direction: incident - normal * (2.0 * incident.dot(&normal)),
        }
    }

    pub fn create_transmission(
        normal: Vector3,
        incident: Vector3,
        intersection: Point,
        bias: Distance,
        index: f32,
    ) -> Option<Self> {
        let mut i_n = incident.dot(&normal);
        let is_into = i_n > 0.0;
        let (eta, n) = if is_into {
            (index as f64, -normal)
        } else {
            i_n = -i_n; // side effect
            (1.0 / (index as f64), normal)
        };
        let i_n_2 = i_n * i_n;
        let i = incident;
        let k = 1.0 - (1.0 - i_n_2) * eta * eta;
        if k < 0.0 {
            None
        } else {
            let t = (i + n * i_n) * eta - n * k.sqrt();
            Some(Ray {
                origin: intersection + ((-n) * bias),
                direction: t.normalize(),
            })
        }
    }
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Distance>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
    fn get_material(&self) -> &Material;
}

pub trait Light {
    fn intensity(&self, hit_point: &Point) -> f32;
    fn distance(&self, hit_point: &Point) -> Distance;
    fn color(&self) -> Color;
    fn direction_from(&self, hit_point: &Point) -> Vector3;
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub item: &'a dyn Intersectable,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f64, item: &dyn Intersectable) -> Intersection {
        Intersection { distance, item }
    }
}

pub fn trace<'a>(scene: &'a Scene, ray: &Ray) -> Option<Intersection<'a>> {
    scene
        .items
        .iter()
        .filter_map(|i| i.intersect(ray).map(|d| Intersection::new(d, i.as_ref())))
        .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
}

pub fn render(scene: &Scene) -> DynamicImage {
    let image = ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
        let ray = Ray::new_prime(x, y, scene);
        if let Some(intersection) = trace(scene, &ray) {
            Rgba::from(get_color(scene, &ray, &intersection, 0).clamp().to_rgba8())
        } else {
            Rgba::from([0, 0, 0, 0])
        }
    });
    DynamicImage::ImageRgba8(image)
}

pub fn cast_ray(scene: &Scene, ray: &Ray, depth: usize) -> Color {
    if depth >= MAX_RECURSION {
        return Color::black();
    }

    let intersection = trace(scene, ray);
    intersection
        .map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or_else(Color::black)
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: usize) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.item.surface_normal(&hit_point);
    match intersection.item.get_material().surface {
        SurfaceType::Diffuse => shader_diffuse(scene, intersection.item, hit_point, surface_normal),
        SurfaceType::Reflective { reflectivity } => {
            let mut color = shader_diffuse(scene, intersection.item, hit_point, surface_normal);
            let reflection_ray =
                Ray::create_reflection(surface_normal, ray.direction, hit_point, SHADOW_BIAS);
            color = color * (1.0 - reflectivity);
            color += cast_ray(scene, &reflection_ray, depth + 1) * reflectivity;
            color
        }
        SurfaceType::Refractive {
            index,
            transparency,
        } => {
            let mut refraction_color = Color::black();
            let uv = intersection.item.texture_coords(&hit_point);
            let surface_color = intersection.item.get_material().color.color(&uv);
            let kr = fresnel(ray.direction, surface_normal, index) as f32;

            if kr < 1.0 {
                let transmission_ray = Ray::create_transmission(
                    surface_normal,
                    ray.direction,
                    hit_point,
                    SHADOW_BIAS,
                    index,
                )
                .expect("gettting trans ray");
                refraction_color = cast_ray(scene, &transmission_ray, depth + 1);
            }
            // println!(
            //     "hit:{:?}, in:{:?}, n:{:?} -> {:?}",
            //     hit_point, ray.direction, surface_normal, transmission_ray.direction
            // );

            let reflection_ray =
                Ray::create_reflection(surface_normal, ray.direction, hit_point, SHADOW_BIAS);
            let reflection_color = cast_ray(scene, &reflection_ray, depth + 1);
            let mut color = reflection_color * kr + refraction_color * (1.0 - kr);
            // println!("d: {} refc:{:?}, surfacec:{:?}", depth, color, surface_color);
            color = color * transparency * surface_color;
            color
        }
    }
}

fn shader_diffuse(
    scene: &Scene,
    item: &dyn Intersectable,
    hit_point: Point,
    surface_normal: Vector3,
) -> Color {
    let uv = item.texture_coords(&hit_point);
    let color = scene
        .lights
        .iter()
        .map(|light| color_from_light(scene, light.as_ref(), hit_point, surface_normal))
        .sum::<Color>()
        * item.get_material().albedo
        / std::f32::consts::PI;
    item.get_material().color.color(&uv) * color
}

fn color_from_light(
    scene: &Scene,
    light: &dyn Light,
    hit_point: Point,
    surface_normal: Vector3,
) -> Color {
    let dir = light.direction_from(&hit_point);
    let theta = surface_normal.dot(&dir) as f32;
    let shadow_ray = Ray {
        origin: hit_point + surface_normal * SHADOW_BIAS,
        direction: dir,
    };
    let shadow_intersection = trace(scene, &shadow_ray);
    let is_in_light = shadow_intersection.is_none()
        || shadow_intersection.unwrap().distance > light.distance(&hit_point);
    light.color()
        * if is_in_light {
            light.intensity(&hit_point) * theta
        } else {
            0.0
        }
}

fn fresnel(incident: Vector3, normal: Vector3, index: f32) -> f64 {
    let i_dot_n = incident.dot(&normal);
    let mut eta_i = 1.0;
    let mut eta_t = index as f64;
    if i_dot_n > 0.0 {
        eta_i = eta_t;
        eta_t = 1.0;
    }

    let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
    if sin_t > 1.0 {
        //Total internal reflection
        return 1.0;
    } else {
        let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
        let cos_i = cos_t.abs();
        let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
        let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
        return (r_s * r_s + r_p * r_p) / 2.0;
    }
}
