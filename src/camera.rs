use std::f64;
use rand::Rng;
use crate::{ray::Ray, vec3::Vec3};

fn random_in_unit_disk() -> Vec3 {
    let mut rng = rand::thread_rng();
    let unit = Vec3::new(1.0, 1.0, 0.0);
    loop {
        let p = 2.0 * Vec3::new(rng.gen::<f64>(), rng.gen::<f64>(), 0.0) - unit;
        if p.dot(p) < 1.0 {
            return p
        }
    }
}

pub struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, view_up: Vec3, vertical_fov: f64, aspect: f64, aperture: f64, focus_dist: f64) -> Self {
        let theta = vertical_fov * std::f64::consts::PI / 180.0;
        let half_height = focus_dist * f64::tan(theta / 2.0);
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = view_up.cross(w).unit();
        let v = w.cross(u);
        Camera {
            origin: look_from,
            lower_left_corner: look_from - half_width * u - half_height * v - focus_dist * w,
            horizontal: u * 2.0 * half_width,
            vertical: v * half_height * 2.0,
            u, v,
            lens_radius: aperture / 2.0
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius ;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + self.horizontal * s + self.vertical * t - self.origin - offset)
    }
}