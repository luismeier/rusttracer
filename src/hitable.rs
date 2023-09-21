use crate::{ray::Ray, Vec3, material::Material};
use Vec3 as Point3;

/**
 * A record of a hit. Point of hit, normal to hit, t?
 */
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64
}

impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64) -> Self {
        Self { p, normal, t}
    }
}

pub trait Hitable: {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)>;
}
