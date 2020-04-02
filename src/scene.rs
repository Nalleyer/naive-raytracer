use std::ops::{Add, AddAssign, Mul, Div};
use crate::math::{Point, Vector3};
use crate::rendering::{Ray, Intersectable};

pub type Distance = f64;

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
            255u8
        ]
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
    type Output = Self ;
    fn add(self, other: Self) -> Self {
        Self {
            r: self.r+ other.r,
            g: self.g+ other.g,
            b: self.b+ other.b,
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

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point,
    pub radius: Distance,
    pub color: Color,
    pub albedo: f32,
}

#[derive(Debug, Clone)]
pub struct Plane {
    pub pos: Point,
    pub normal: Vector3,
    pub color: Color,
    pub albedo: f32,
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

    fn get_color(&self) -> Color {
        self.color
    }

    fn surface_normal(&self, hit_point: &Point) -> Vector3 {
        (*hit_point - self.center).normalize()
    }

    fn albedo(&self) -> f32 {
        self.albedo
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

    fn get_color(&self) -> Color {
        self.color
    }

    fn surface_normal(&self, _hit_point: &Point) -> Vector3 {
        -self.normal
    }

    fn albedo(&self) -> f32 {
        self.albedo
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