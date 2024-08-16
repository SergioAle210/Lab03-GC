use crate::color::Color;
use crate::RayIntersect;
use nalgebra_glm::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub color: Color,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, color: Color) -> Self {
        Self {
            center,
            radius,
            color,
        }
    }
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> bool {
        let l = self.center - *ray_origin;
        let tca = l.dot(ray_direction);
        let d2 = l.dot(&l) - tca * tca;
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return false;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return false;
        }

        true
    }
}
