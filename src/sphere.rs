use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }

    pub fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        // Normalizar el vector desde el centro de la esfera al punto de intersección
        let r = (point - self.center).normalize();

        // Calcular θ (theta) y φ (phi)
        let theta = r.z.atan2(r.x); // Arctan(z / x)
        let phi = r.y.asin(); // Arcsin(y)

        // Convertir los ángulos a coordenadas UV
        let u = 0.5 + theta / (2.0 * std::f32::consts::PI);
        let v = 0.5 - phi / std::f32::consts::PI;

        (u, v)
    }
}

impl RayIntersect for Sphere {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let l = self.center - *ray_origin;
        let tca = l.dot(ray_direction);
        let d2 = l.dot(&l) - tca * tca;
        let radius2 = self.radius * self.radius;

        if d2 > radius2 {
            return Intersect::empty();
        }

        let thc = (radius2 - d2).sqrt();
        let t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 && t1 < 0.0 {
            return Intersect::empty();
        }

        let distance = if t0 < 0.0 { t1 } else { t0 };

        // Calcular el punto de impacto
        let point = ray_origin + ray_direction * distance;

        // Calcular la normal en el punto de impacto
        let normal = (point - self.center).normalize();

        // Obtener las coordenadas UV
        let (u, v) = self.get_uv(&point);

        Intersect::new(point, normal, distance, self.material.clone(), u, v)
    }
}
