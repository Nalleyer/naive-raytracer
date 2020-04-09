use crate::color::Color;
use crate::math::{Point, Vector3};
use crate::scene::{
    material::{Material, TextureCoords, SurfaceType},
    Distance, Scene,
};

pub const SHADOW_BIAS: Distance = 1e-13;
pub const MAX_RECURSION: usize = 50;

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

    pub fn crate_reflection(normal: Vector3, incident: Vector3, intersection: Point, bias: Distance) -> Self {
    Ray {
        origin: intersection + (normal * bias),
        direction: incident - normal * (2.0 * incident.dot(&normal)),
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
        return Color::black()
    }

    let intersection = trace(scene, ray);
    intersection.map(|i| get_color(scene, &ray, &i, depth))
        .unwrap_or(Color::black())
}

fn get_color(scene: &Scene, ray: &Ray, intersection: &Intersection, depth: usize) -> Color {
    let hit_point = ray.origin + (ray.direction * intersection.distance);
    let surface_normal = intersection.item.surface_normal(&hit_point);
    let mut color = shader_diffuse(scene, intersection.item, hit_point, surface_normal);
    if let SurfaceType::Reflective { reflectivity } = intersection.item.get_material().surface {
        let reflection_ray = Ray::crate_reflection(surface_normal, ray.direction, hit_point, SHADOW_BIAS);
        color = color * (1.0 - reflectivity);
        color += cast_ray(scene, &reflection_ray, depth + 1) * reflectivity;
    }
    color
}

fn shader_diffuse(scene: &Scene, item: &dyn Intersectable, hit_point: Point, surface_normal: Vector3) -> Color {
    let uv = item.texture_coords(&hit_point);
    let color = scene
        .lights
        .iter()
        .map(|light| {
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
        })
        .sum::<Color>()
        * item.get_material().albedo
        / std::f32::consts::PI;
    item.get_material().color.color(&uv) * color
}
