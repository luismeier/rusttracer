use crate::{
    ray::Ray,
    vec3::{random_in_unit_sphere, Vec3},
};
use rand::Rng;
use Vec3 as Color;

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(v: Vec3, n: Vec3, etai_over_etat: f64) -> Option<Vec3> {
    let uv = v.unit();
    let dt = uv.dot(n);
    let discriminant = 1.0 - etai_over_etat * etai_over_etat * (1.0 - dt * dt);
    if discriminant > 0.0 {
        Some((uv - n * dt) * etai_over_etat - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlik(cosine: f64, refraction: f64) -> f64 {
    let r0 = ((1.0 - refraction) / (1.0 + refraction)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { attenuation: Color },
    Metal { attenuation: Color, fuzzines: f64 },
    Dielectric { refraction_idx: f64 },
}

impl Material {
    pub fn scatter(&self, r_in: &Ray, n: Vec3, p: Vec3) -> (Ray, Color, bool) {
        let target = p + n + random_in_unit_sphere();
        match self {
            Material::Lambertian { attenuation } => (Ray::new(p, target - p), *attenuation, true),
            Material::Metal {
                attenuation,
                fuzzines,
            } => {
                let reflected = reflect(r_in.direction().unit(), n);
                let scattered = Ray::new(p, reflected + random_in_unit_sphere() * *fuzzines);
                let b = scattered.direction().dot(n) > 0.0;
                (scattered, *attenuation, b)
            }
            Material::Dielectric { refraction_idx } => {
                let reflected = reflect(r_in.direction(), n);
                let (outward_normal, etai_over_etat, cosine) = if r_in.direction().dot(n) > 0.0 {
                    (
                        -n,
                        *refraction_idx,
                        refraction_idx * r_in.direction().dot(n) / r_in.direction().length(),
                    )
                } else {
                    (
                        n,
                        1.0 / refraction_idx,
                        -(r_in.direction().dot(n)) / r_in.direction().length(),
                    )
                };
                let scattered = match refract(r_in.direction(), outward_normal, etai_over_etat) {
                    Some(refracted) => {
                        let reflect_prob = schlik(cosine, *refraction_idx);

                        let mut rng = rand::thread_rng();
                        if rng.gen::<f64>() < reflect_prob {
                            Ray::new(p, reflected)
                        } else {
                            Ray::new(p, refracted)
                        }
                    }
                    None => Ray::new(p, reflected),
                };
                let attenuation = Vec3::new(1.0, 1.0, 1.0);
                (scattered, attenuation, true)
            }
        }
    }
}
