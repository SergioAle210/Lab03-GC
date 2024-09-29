use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use nalgebra_glm::{rotate, translate, Mat4, Vec3};
use std::f32::consts::PI;

pub struct Cuboid {
    pub center: Vec3,
    pub width: f32,
    pub height: f32,
    pub depth: f32,
    pub material: Material,
}

impl Cuboid {
    pub fn new(center: Vec3, width: f32, height: f32, depth: f32, material: Material) -> Self {
        Self {
            center,
            width,
            height,
            depth,
            material,
        }
    }

    pub fn get_uv(&self, point: &Vec3) -> (f32, f32) {
        let local_point = point - self.center;
        let u = (local_point.x / self.width) + 0.5;
        let v = (local_point.y / self.height) + 0.5;
        (u, v)
    }
}

impl RayIntersect for Cuboid {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let min = self.center - Vec3::new(self.width / 2.0, self.height / 2.0, self.depth / 2.0);
        let max = self.center + Vec3::new(self.width / 2.0, self.height / 2.0, self.depth / 2.0);

        let mut t_min = (min.x - ray_origin.x) / ray_direction.x;
        let mut t_max = (max.x - ray_origin.x) / ray_direction.x;

        if t_min > t_max {
            std::mem::swap(&mut t_min, &mut t_max);
        }

        let mut t_ymin = (min.y - ray_origin.y) / ray_direction.y;
        let mut t_ymax = (max.y - ray_origin.y) / ray_direction.y;

        if t_ymin > t_ymax {
            std::mem::swap(&mut t_ymin, &mut t_ymax);
        }

        if t_min > t_ymax || t_ymin > t_max {
            return Intersect::empty();
        }

        if t_ymin > t_min {
            t_min = t_ymin;
        }
        if t_ymax < t_max {
            t_max = t_ymax;
        }

        let mut t_zmin = (min.z - ray_origin.z) / ray_direction.z;
        let mut t_zmax = (max.z - ray_origin.z) / ray_direction.z;

        if t_zmin > t_zmax {
            std::mem::swap(&mut t_zmin, &mut t_zmax);
        }

        if t_min > t_zmax || t_zmin > t_max {
            return Intersect::empty();
        }

        if t_zmin > t_min {
            t_min = t_zmin;
        }
        if t_zmax < t_max {
            t_max = t_zmax;
        }

        if t_min < 0.0 {
            return Intersect::empty();
        }

        let hit_point = ray_origin + ray_direction * t_min;
        let normal = (hit_point - self.center).normalize();
        let (u, v) = self.get_uv(&hit_point);

        Intersect::new(hit_point, normal, t_min, self.material.clone(), u, v)
    }
}
