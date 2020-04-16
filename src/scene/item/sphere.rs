use crate::math::{Point, Vector3};
use crate::rendering::{Intersectable, Ray};
use crate::scene::{
    material::{Material, TextureCoords},
    Distance,
};

pub struct Sphere {
    pub center: Point,
    pub radius: Distance,
    pub material: Box<dyn Material + Send + Sync>,
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
            let t0 = -iq_len + os_on_ray;
            let t1 = iq_len + os_on_ray;
            if t0 < 0f64 && t1 < 0f64 {
                None
            } else if t0 < 0.0 {
                Some(t1)
            } else if t1 < 0.0 {
                Some(t0)
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
        self.material.as_ref()
    }
}
