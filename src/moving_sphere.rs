use std::sync::Arc;

use crate::*;

pub struct MovingSphere {
    center0: Vector3,
    center1: Vector3,
    time0: f32,
    time1: f32,
    radius: f32,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new<T: Material + 'static, U: Into<Arc<T>>>(
        center0: &Vector3,
        center1: &Vector3,
        time0: f32,
        time1: f32,
        radius: f32,
        material: U,
    ) -> MovingSphere {
        MovingSphere {
            center0: *center0,
            center1: *center1,
            time0,
            time1,
            radius,
            material: material.into(),
        }
    }

    fn center(&self, time: f32) -> Vector3 {
        let t = (time - self.time0) / (self.time1 - self.time0);
        (1.0 - t) * self.center0 + t * self.center1
    }
}

unsafe impl Sync for MovingSphere {}

unsafe impl Send for MovingSphere {}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let center = self.center(ray.time());
        let co = ray.origin() - center;
        let a = ray.direction().dot(ray.direction());
        let b = 2.0 * ray.direction().dot(&co);
        let c = co.dot(&co) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return None;
        }

        let t = (|| {
            let t = (-b + discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                return Some(t);
            }
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                return Some(t);
            }
            None
        })();
        match t {
            Some(t) => {
                let point = ray.point_at(t);
                let normal = &(point - center) / self.radius;
                let (u, v) = common::get_sphere_uv(&normal);
                Some(HitRecord {
                    t,
                    position: ray.point_at(t),
                    normal,
                    material: self.material.clone(),
                    u,
                    v,
                })
            }
            None => None,
        }
    }

    fn bounding_box(&self, t0: f32, t1: f32) -> Option<AABB> {
        let half = Vector3::new(self.radius, self.radius, self.radius);
        let center = self.center(t0);
        let aabb0 = AABB::new(center - half, center + half);
        let center = self.center(t1);
        let aabb1 = AABB::new(center - half, center + half);
        Some(aabb0.surrounding(&aabb1))
    }
}
