use crate::color::Color;
use crate::math::{Point, Vector3};
use crate::rendering::{Ray, SHADOW_BIAS};
use image::ImageBuffer;

#[derive(Clone)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
    Refractive { index: f32, transparency: f32 },
}

#[derive(Debug)]
pub struct Scatter {
    pub ray: Option<Ray>,
    pub color: Color,
}

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        normal: &Vector3,
        hit_point: &Point,
        uv: &TextureCoords,
    ) -> Scatter;
    fn emmit(&self, ray: &Ray, hit_point: &Point) -> Color;
}

// #[derive(Clone)]
// pub struct Material {
//     pub color: Coloration,
//     pub albedo: f32,
//     pub surface: SurfaceType,
// }

#[derive(Clone)]
pub struct UniversalMaterial {
    pub color: Coloration,
    pub albedo: f32,
    pub index: f32,
    pub transparency: f32,
    pub reflectivity: f32,
    pub emmit: f32,
    pub is_light: bool,
}

impl Material for UniversalMaterial {
    fn scatter(
        &self,
        ray: &Ray,
        normal: &Vector3,
        hit_point: &Point,
        uv: &TextureCoords,
    ) -> Scatter {
        let target = *hit_point + *normal + random_in_unit_sphere();
        let new_v = (target - *hit_point).normalize();
        Scatter {
            ray: if self.is_light {
                None
            } else {
                Some(Ray {
                    origin: *hit_point + new_v * SHADOW_BIAS,
                    direction: new_v,
                })
            },
            color: self.color.color(uv) * self.albedo,
        }
    }

    fn emmit(&self, ray: &Ray, hit_point: &Point) -> Color {
        Color {
            r: 1.0,
            g: 1.0,
            b: 0.0,
        } * self.emmit
    }
}

#[derive(Clone)]
pub enum Coloration {
    Color(Color),
    Texture(Texture),
}

#[derive(Clone)]
pub struct Texture {
    pub image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale: f32,
}

pub struct TextureCoords {
    pub u: f32,
    pub v: f32,
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match self {
            Self::Color(c) => *c,
            Self::Texture(tex) => {
                let u = wrap(
                    (texture_coords.u + tex.offset_x) / tex.scale,
                    tex.image.width(),
                );
                let v = wrap(
                    (texture_coords.v + tex.offset_y) / tex.scale,
                    tex.image.height(),
                );
                Color::from_rgba8(tex.image.get_pixel(u, v).0)
            }
        }
    }
}

fn random_in_unit_sphere() -> Vector3 {
    loop {
        let p = Vector3::new(
            rand::random::<f64>(),
            rand::random::<f64>(),
            rand::random::<f64>(),
        ) * 2.0
            - Vector3::new(1.0, 1.0, 1.0);
        if p.norm() <= 1.0 {
            return p;
        }
    }
}

fn reflect(v: &Vector3, normal: &Vector3) -> Vector3 {
    *v - *normal * 2.0 * v.dot(normal)
}

fn refract(v: &Vector3, normal: &Vector3, eta: f64) -> Option<Vector3> {
    let uv = v.normalize();
    let dt = uv.dot(normal);
    let discriminant = 1.0 - eta * eta * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - *normal * dt) * eta - *normal * discriminant.sqrt())
    } else {
        None
    }
}
