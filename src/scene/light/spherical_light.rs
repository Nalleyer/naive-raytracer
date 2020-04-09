use crate::color::Color;
use crate::math::{Point, Vector3};
use crate::rendering::Light;
use crate::scene::Distance;

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
