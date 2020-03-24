use std::ops::{Add, AddAssign};
use crate::math::{Point, Vector3};
use crate::rendering::{Ray, Intersectable};

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

#[derive(Debug)]
pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub color: Color,
}

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub items: Vec<Box<Sphere>>,
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // 定义l是发射点到球心的向量
        let l: Vector3 = self.center - ray.origin;
        // 为了算球心到射线的距离，先算另一个直角边。它的长度是l在ray上的投影
        // |adj2| = ray cosθ = l . ray / |ray| = l . ray
        let adj2 = l.dot(&ray.direction);
        let d2 = l.dot(&l) - (adj2 * adj2);
        d2 < (self.radius * self.radius)
    }
}