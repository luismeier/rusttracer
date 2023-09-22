mod camera;
mod hitable;
mod hitable_list;
mod material;
mod ray;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hitable_list::HitableList;
use crate::material::Material;
use crate::sphere::Sphere;
use crate::vec3::write_color;
use crate::vec3::Vec3;
use hitable::Hitable;
use rand::Rng;
use ray::Ray;

use Vec3 as Color;

fn linear_to_gamma(linear: f64) -> f64 {
    return linear.sqrt();
}

fn ray_color(ray: &Ray, world: &dyn Hitable, depht: i64) -> Color {
    // Check if we hit something
    match world.hit(ray, 0.001, std::f64::INFINITY) {
        // there is a hit
        Some((hit_record, material)) => {
            let n = hit_record.normal;
            let p = hit_record.p;
            let (scattered, attenuation, b) = material.scatter(ray, n, p);

            // Check if we have reached max depht
            // this is to prevent endless bounces of light
            if depht <= 0 || b == false {
                return Color::new(0.0, 0.0, 0.0);
            }

            attenuation * ray_color(&scattered, world, depht - 1)
        }

        // no hit
        None => {
            let unit_direction = ray.direction().unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WITH: i64 = 900;
    const IMAGE_HEIGHT: i64 = (IMAGE_WITH as f64 / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i64 = 10;
    const MAX_DEPHT: i64 = 50;

    let material_ground = Material::Lambertian {
        attenuation: Vec3::new(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        attenuation: Vec3::new(0.1, 0.2, 0.5),
    };
    let material_right = Material::Metal {
        attenuation: Vec3::new(0.8, 0.6, 0.2),
        fuzzines: 0.0,
    };
    let material_left = Material::Dielectric {
        refraction_idx: 1.5,
    };

    // World
    let world = HitableList::new(vec![
        Box::new(Sphere::new(
            Vec3::new(0.0, -100.5, -1.0),
            100.0,
            material_ground,
        )),
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5, material_center)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5, material_left)),
        Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.4, material_left)),
        Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5, material_right)),
    ]);

    // Camera
    let camera = Camera::new();

    println!("P3\n{} {}\n255\n", IMAGE_WITH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WITH {
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            for _s in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rng.gen::<f64>()) / IMAGE_WITH as f64;
                let v = (j as f64 + rng.gen::<f64>()) / IMAGE_HEIGHT as f64;
                let ray = camera.get_ray(u, v);

                pixel_color += ray_color(&ray, &world, MAX_DEPHT);
            }
            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
    }
    eprint!("\nAll Done!");
}
