use crate::math::{Point, Vector3};
use crate::rendering::{Intersectable, Ray};
use std::ops::{Add, AddAssign, Div, Mul};

pub type Distance = f64;

pub const SHADOW_BIAS: f64 = 1e-13;

use image::ImageBuffer;

#[derive(Debug, PartialEq, Default, Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn to_rgba8(self) -> [u8; 4] {
        [
            (self.r * 255f32) as u8,
            (self.g * 255f32) as u8,
            (self.b * 255f32) as u8,
            255u8,
        ]
    }

    pub fn from_rgba8(rgba8: &[u8; 4]) -> Self {
        Color {
            r: rgba8[0] as f32 / 255f32,
            g: rgba8[1] as f32 / 255f32,
            b: rgba8[2] as f32 / 255f32,
        }
    }

    pub fn clamp(&self) -> Color {
        Color {
            r: self.r.min(1.0).max(0.0),
            b: self.b.min(1.0).max(0.0),
            g: self.g.min(1.0).max(0.0),
        }
    }
}

impl Add for Color {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            r: self.r + other.r,
            g: self.g + other.g,
            b: self.b + other.b,
        };
    }
}

impl Div<f32> for Color {
    type Output = Self;
    fn div(self, other: f32) -> Self::Output {
        Self::Output {
            r: self.r / other,
            b: self.b / other,
            g: self.g / other,
        }
    }
}

impl std::iter::Sum for Color {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Color::default(), Add::add)
    }
}

impl Mul<f32> for Color {
    type Output = Self;
    fn mul(self, other: f32) -> Self {
        Self {
            r: self.r * other,
            g: self.g * other,
            b: self.b * other,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Self;
    fn mul(self, other: Color) -> Self {
        Self {
            r: self.r * other.r,
            g: self.g * other.g,
            b: self.b * other.b,
        }
    }
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: Distance,
    pub material: Material,
}

#[derive(Clone)]
pub struct Plane {
    pub pos: Point,
    pub normal: Vector3,
    pub material: Material,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: Distance,
    pub items: Vec<Box<dyn Intersectable>>,
    pub lights: Vec<Box<dyn Light>>,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Distance> {
        // S: 球心 O: ray起点 I: 交点（如果有） Q: 从S引垂线交ray于Q
        let os: Vector3 = self.center - ray.origin;
        // 为了算球心到射线的距离d，先算另一个直角边。它的长度是os在ray上的投影
        // os_on_ray = ray cosθ = os . ray / |ray| = os . ray
        let os_on_ray = os.dot(&ray.direction);
        let d2 = os.dot(&os) - (os_on_ray * os_on_ray);
        let r2 = self.radius * self.radius;
        if d2 > r2 {
            None
        } else {
            let iq_len = (r2 - d2).sqrt();
            let t0 = iq_len + os_on_ray;
            let t1 = -iq_len + os_on_ray;
            if t0 < 0f64 || t1 < 0f64 {
                None
            } else {
                Some(t0.min(t1))
            }
        }
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let p = *hit_point - self.center;
        let phi = (p.z).atan2(p.x);
        let theta = (p.y / self.radius).acos();
        TextureCoords {
            u: (1.0 + phi) as f32 / std::f32::consts::PI * 0.5,
            v: theta as f32 / std::f32::consts::PI,
        }
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Option<Distance> {
        let normal = &self.normal;
        let denom = normal.dot(&ray.direction);
        if denom > 1e-6 {
            let v = self.pos - ray.origin;
            let distance = v.dot(&normal) / denom;
            if distance >= 0.0 {
                return Some(distance);
            }
        }
        None
    }

    fn surface_normal(&self, _hit_point: &Point) -> Vector3 {
        -self.normal
    }

    fn texture_coords(&self, hit_point: &Point) -> TextureCoords {
        let mut x_axis = self.normal.cross(&Vector3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        });
        if x_axis.length() == 0.0 {
            x_axis = self.normal.cross(&Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            });
        }
        let y_axis = self.normal.cross(&x_axis);

        let p: Vector3 = *hit_point - self.pos;

        TextureCoords {
            u: p.dot(&x_axis) as f32,
            v: p.dot(&y_axis) as f32,
        }
    }

    fn get_material(&self) -> &Material {
        &self.material
    }
}

pub trait Light {
    fn intensity(&self, hit_point: &Point) -> f32;
    fn distance(&self, hit_point: &Point) -> Distance;
    fn color(&self) -> Color;
    fn direction_from(&self, hit_point: &Point) -> Vector3;
}

#[derive(Debug)]
pub struct DirectionalLight {
    pub direction: Vector3,
    pub color: Color,
    pub intensity: f32,
}

impl Light for DirectionalLight {
    fn intensity(&self, _hit_point: &Point) -> f32 {
        self.intensity
    }

    fn direction_from(&self, _hit_point: &Point) -> Vector3 {
        -self.direction
    }

    fn color(&self) -> Color {
        self.color
    }

    fn distance(&self, _hit_point: &Point) -> Distance {
        std::f64::INFINITY
    }
}

#[derive(Debug)]
pub struct SphericalLight {
    pub position: Point,
    pub color: Color,
    pub intensity: f32,
}

impl Light for SphericalLight {
    fn intensity(&self, hit_point: &Point) -> f32 {
        let r2 = (self.position - *hit_point).norm() as f32;
        self.intensity / (r2 * 4.0 * std::f32::consts::PI)
    }

    fn direction_from(&self, hit_point: &Point) -> Vector3 {
        (self.position - *hit_point).normalize()
    }

    fn color(&self) -> Color {
        self.color
    }

    fn distance(&self, hit_point: &Point) -> Distance {
        (self.position - *hit_point).length()
    }
}

#[derive(Clone)]
pub struct Texture {
    pub image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale: f32,
}

#[derive(Clone)]
pub enum Coloration {
    Color(Color),
    Texture(Texture),
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
                let u = wrap((texture_coords.u + tex.offset_x) / tex.scale, tex.image.width());
                let v = wrap((texture_coords.v + tex.offset_y) / tex.scale, tex.image.height());
                Color::from_rgba8(&tex.image.get_pixel(u, v).0)
            }
        }
    }
}

#[derive(Clone)]
pub struct Material {
    pub color: Coloration,
    pub albedo: f32,
}

pub struct TextureCoords {
    pub u: f32,
    pub v: f32,
}
