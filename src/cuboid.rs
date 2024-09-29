use crate::material::Material;
use crate::ray_intersect::{Intersect, RayIntersect};
use nalgebra_glm::Vec3;

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

    pub fn get_uv(&self, point: &Vec3, normal: &Vec3) -> (f32, f32) {
        // Use different UV calculations for each face
        if normal.x > 0.9 {
            // Right face
            let u = (point.z - (self.center.z - self.depth / 2.0)) / self.depth;
            let v = (point.y - (self.center.y - self.height / 2.0)) / self.height;
            return (1.0 - u, v);
        } else if normal.x < -0.9 {
            // Left face
            let u = (point.z - (self.center.z - self.depth / 2.0)) / self.depth;
            let v = (point.y - (self.center.y - self.height / 2.0)) / self.height;
            return (u, v);
        } else if normal.y > 0.9 {
            // Top face
            let u = (point.x - (self.center.x - self.width / 2.0)) / self.width;
            let v = (point.z - (self.center.z - self.depth / 2.0)) / self.depth;
            return (u, 1.0 - v);
        } else if normal.y < -0.9 {
            // Bottom face
            let u = (point.x - (self.center.x - self.width / 2.0)) / self.width;
            let v = (point.z - (self.center.z - self.depth / 2.0)) / self.depth;
            return (u, v);
        } else if normal.z > 0.9 {
            // Front face
            let u = (point.x - (self.center.x - self.width / 2.0)) / self.width;
            let v = (point.y - (self.center.y - self.height / 2.0)) / self.height;
            return (u, 1.0 - v);
        } else if normal.z < -0.9 {
            // Back face
            let u = (point.x - (self.center.x - self.width / 2.0)) / self.width;
            let v = (point.y - (self.center.y - self.height / 2.0)) / self.height;
            return (1.0 - u, 1.0 - v);
        }

        // Return (0.0, 0.0) if somehow no valid normal was found (should not happen)
        (0.0, 0.0)
    }
}

impl RayIntersect for Cuboid {
    fn ray_intersect(&self, ray_origin: &Vec3, ray_direction: &Vec3) -> Intersect {
        let min = self.center - Vec3::new(self.width / 2.0, self.height / 2.0, self.depth / 2.0);
        let max = self.center + Vec3::new(self.width / 2.0, self.height / 2.0, self.depth / 2.0);

        // Calculate intersection distances for the x-axis
        let mut tmin = (min.x - ray_origin.x) / ray_direction.x;
        let mut tmax = (max.x - ray_origin.x) / ray_direction.x;

        if tmin > tmax {
            std::mem::swap(&mut tmin, &mut tmax);
        }

        // Calculate intersection distances for the y-axis
        let mut tymin = (min.y - ray_origin.y) / ray_direction.y;
        let mut tymax = (max.y - ray_origin.y) / ray_direction.y;

        if tymin > tymax {
            std::mem::swap(&mut tymin, &mut tymax);
        }

        // If there is no overlap between x and y intervals, return no intersection
        if (tmin > tymax) || (tymin > tmax) {
            return Intersect::empty();
        }

        // Adjust tmin to be the maximum of the two, ensuring it's the farthest near intersection
        if tymin > tmin {
            tmin = tymin;
        }

        // Adjust tmax to be the minimum of the two, ensuring it's the closest far intersection
        if tymax < tmax {
            tmax = tymax;
        }

        // Calculate intersection distances for the z-axis
        let mut tzmin = (min.z - ray_origin.z) / ray_direction.z;
        let mut tzmax = (max.z - ray_origin.z) / ray_direction.z;

        if tzmin > tzmax {
            std::mem::swap(&mut tzmin, &mut tzmax);
        }

        // Check for overlap with the z-axis interval
        if (tmin > tzmax) || (tzmin > tmax) {
            return Intersect::empty();
        }

        // Further adjust tmin and tmax based on the z values
        if tzmin > tmin {
            tmin = tzmin;
        }

        // If tmax is not used after this point, you can omit this check.
        // If it needs to be adjusted correctly, this line ensures the proper overlap.
        // Otherwise, the assignment can be completely removed if it's never utilized:
        // `tmax = tzmax;` can be omitted if not needed for further processing.

        // The actual intersection distance is represented by `tmin`
        let distance = tmin;

        if distance < 0.0 {
            return Intersect::empty();
        }

        // Calculate the intersection point and normal
        let hit_point = ray_origin + ray_direction * distance;
        let mut normal = Vec3::zeros();

        // Determine which face was hit based on the intersection point
        if (hit_point.x - min.x).abs() < f32::EPSILON {
            normal = Vec3::new(-1.0, 0.0, 0.0); // left face
        } else if (hit_point.x - max.x).abs() < f32::EPSILON {
            normal = Vec3::new(1.0, 0.0, 0.0); // right face
        } else if (hit_point.y - min.y).abs() < f32::EPSILON {
            normal = Vec3::new(0.0, -1.0, 0.0); // bottom face
        } else if (hit_point.y - max.y).abs() < f32::EPSILON {
            normal = Vec3::new(0.0, 1.0, 0.0); // top face
        } else if (hit_point.z - min.z).abs() < f32::EPSILON {
            normal = Vec3::new(0.0, 0.0, -1.0); // back face
        } else if (hit_point.z - max.z).abs() < f32::EPSILON {
            normal = Vec3::new(0.0, 0.0, 1.0); // front face
        }

        // Get UV coordinates based on the hit point and face normal
        let (u, v) = self.get_uv(&hit_point, &normal);

        Intersect::new(hit_point, normal, distance, self.material.clone(), u, v)
    }
}
