use crate::{
    ray::Ray,
    vec3::{random_in_unit_sphere, Vec3},
};
use Vec3 as Color;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub enum Material {
    Lambertian { attenuation: Color },
    Metal { attenuation: Color },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, n: Vec3, p: Vec3) -> (Ray, Color, bool) {
        let target = p + n + random_in_unit_sphere();
        match self {
            Material::Lambertian { attenuation } => (Ray::new(p, target - p), *attenuation, true),
            Material::Metal { attenuation } => {
                let reflected = reflect(r_in.direction().unit(), n);
                let scattered = Ray::new(p, reflected);
                let b = scattered.direction().dot(n) > 0.0;
                (scattered, *attenuation, b)
            }
        }
    }
}
