use crate::color::Color;
use crate::math::{Point, Vector3};
use crate::rendering::Light;
use crate::scene::Distance;

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
