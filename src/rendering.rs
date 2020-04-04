use crate::math::{Point, Vector3};
use crate::scene::{Color, Scene, SHADOW_BIAS, TextureCoords, Distance, Material};

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
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Distance>;
    fn surface_normal(&self, hit_point: &Point) -> Vector3;
    fn texture_coords(&self, hit_point: &Point) -> TextureCoords;
    fn get_material(&self) -> &Material;
}

pub struct Intersection<'a> {
    pub distance: f64,
    pub item: &'a Box<dyn Intersectable>,
}

impl<'a> Intersection<'a> {
    pub fn new<'b>(distance: f64, item: &'b Box<dyn Intersectable>) -> Intersection<'b> {
        Intersection { distance, item }
    }
}

pub fn cast_ray<'a>(scene: &'a Scene, ray: &Ray) -> Option<Intersection<'a>> {
    scene
        .items
        .iter()
        .filter_map(|i| i.intersect(ray).map(|d| Intersection::new(d, i)))
        .min_by(|i1, i2| i1.distance.partial_cmp(&i2.distance).unwrap())
}

pub fn render(scene: &Scene) -> DynamicImage {
    let image = ImageBuffer::from_fn(scene.width, scene.height, |x, y| {
        let ray = Ray::new_prime(x, y, scene);
        if let Some(intersection) = cast_ray(scene, &ray) {
            let hit_point = ray.origin + (ray.direction * intersection.distance);
            let surface_normal = intersection.item.surface_normal(&hit_point);
            let uv = intersection.item.texture_coords(&hit_point);
            let color: Color = scene
                .lights
                .iter()
                .map(|light| {
                    let dir = light.direction_from(&hit_point);
                    let theta = surface_normal.dot(&dir) as f32;
                    let shadow_ray = Ray {
                        origin: hit_point + surface_normal * SHADOW_BIAS,
                        direction: dir,
                    };
                    let shadow_intersection = cast_ray(scene, &shadow_ray);
                    let is_in_light = shadow_intersection.is_none() || shadow_intersection.unwrap().distance > light.distance(&hit_point);
                    light.color()
                        * if is_in_light {
                            light.intensity(&hit_point) * theta
                        } else {
                            0.0
                        }
                })
                .sum::<Color>()
                * intersection.item.get_material().albedo
                / std::f32::consts::PI;

            Rgba::from((intersection.item.get_material().color.color(&uv) * color).clamp().to_rgba8())
        } else {
            Rgba::from([0, 0, 0, 0])
        }
    });
    DynamicImage::ImageRgba8(image)
}
