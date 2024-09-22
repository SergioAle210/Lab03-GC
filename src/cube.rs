use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use nalgebra_glm::Vec3;

pub struct Cube {
    pub min_corner: Vec3, // La esquina inferior izquierda del cubo
    pub max_corner: Vec3, // La esquina superior derecha del cubo
    pub material: Material,
}

impl Cube {
    pub fn new(min_corner: Vec3, max_corner: Vec3, material: Material) -> Self {
        Self {
            min_corner,
            max_corner,
            material,
        }
    }

    fn get_normal(&self, point: &Vec3) -> Vec3 {
        let epsilon = 0.001;
        if (point.x - self.min_corner.x).abs() < epsilon {
            return Vec3::new(-1.0, 0.0, 0.0);
        } else if (point.x - self.max_corner.x).abs() < epsilon {
            return Vec3::new(1.0, 0.0, 0.0);
        } else if (point.y - self.min_corner.y).abs() < epsilon {
            return Vec3::new(0.0, -1.0, 0.0);
        } else if (point.y - self.max_corner.y).abs() < epsilon {
            return Vec3::new(0.0, 1.0, 0.0);
        } else if (point.z - self.min_corner.z).abs() < epsilon {
            return Vec3::new(0.0, 0.0, -1.0);
        } else {
            return Vec3::new(0.0, 0.0, 1.0);
        }
    }
}

impl RayIntersect for Cube {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let inv_dir = Vec3::new(
            1.0 / ray_direction.x,
            1.0 / ray_direction.y,
            1.0 / ray_direction.z,
        );
        let mut tmin = (self.min_corner.x - ray_origin.x) * inv_dir.x;
        let mut tmax = (self.max_corner.x - ray_origin.x) * inv_dir.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        let mut tymin = (self.min_corner.y - ray_origin.y) * inv_dir.y;
        let mut tymax = (self.max_corner.y - ray_origin.y) * inv_dir.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty();
        }

        if tymin > tmin {
            tmin = tymin;
        }

        if tymax < tmax {
            tmax = tymax;
        }

        let mut tzmin = (self.min_corner.z - ray_origin.z) * inv_dir.z;
        let mut tzmax = (self.max_corner.z - ray_origin.z) * inv_dir.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty();
        }

        if tzmin > tmin {
            tmin = tzmin;
        }

        if tzmax < tmax {
            tmax = tzmax;
        }

        let distance = tmin;

        if distance < 0.0 {
            return Intersect::empty();
        }

        let point = ray_origin + ray_direction * distance;
        let normal = self.get_normal(&point);

        // Para un cubo no utilizamos coordenadas UV reales, pero devolvemos valores genéricos
        Intersect::new(point, normal, distance, self.material.clone(), 0.0, 0.0)
    }
}
