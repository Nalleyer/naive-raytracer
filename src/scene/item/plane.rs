use crate::math::{Point, Vector3};
use crate::rendering::{Intersectable, Ray};
use crate::scene::{
    material::{Material, TextureCoords},
    Distance,
};

#[derive(Clone)]
pub struct Plane {
    pub pos: Point,
    pub normal: Vector3,
    pub material: Material,
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
