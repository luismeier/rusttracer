use crate::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
    vec3::{dot, Vec3},
};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Self { center, radius }
    }

    pub fn center(&self) -> &Vec3 {
        &self.center
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().squared_length();
        let half_b = dot(&oc, &ray.direction());
        let c = oc.squared_length() - self.radius * self.radius;

        let discriminant: f64 = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        // Find the neares root that lies in the accaptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = ray.at(root);
        let hr = HitRecord::new(p, (p - *self.center()) / self.radius, root);
        return Some(hr);
    }
}
