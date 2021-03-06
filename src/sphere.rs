use std::sync::Arc;

use crate::*;

pub struct Sphere {
    center: Vector3,
    radius: f32,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new<T: Material + 'static, U: Into<Arc<T>>>(
        center: &Vector3,
        radius: f32,
        material: U,
    ) -> Sphere {
        Sphere {
            center: *center,
            radius,
            material: material.into(),
        }
    }

    pub fn center(&self) -> &Vector3 {
        &self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

unsafe impl Send for Sphere {}

unsafe impl Sync for Sphere {}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let co = ray.origin() - self.center; // center to origin
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(&co);
        let c = co.dot(&co) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        let t = {
            || -> Option<f32> {
                if discriminant < 0.0 {
                    return None;
                }
                let t = (-b - discriminant.sqrt()) / (2.0 * a);
                if t > t_min && t < t_max {
                    return Some(t);
                }
                let t = (-b + discriminant.sqrt()) / (2.0 * a);
                if t > t_min && t < t_max {
                    return Some(t);
                }
                None
            }()
        };

        match t {
            Some(t) => {
                let position = ray.point_at(t);
                let normal = &(position - self.center) / self.radius;
                let (u, v) = common::get_sphere_uv(&normal);
                Some(HitRecord {
                    t,
                    position,
                    normal,
                    material: self.material.clone(),
                    u,
                    v,
                })
            }
            None => None,
        }
    }

    fn bounding_box(&self, _t0: f32, _t1: f32) -> Option<AABB> {
        let half = Vector3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(self.center - half, self.center + half))
    }
}
