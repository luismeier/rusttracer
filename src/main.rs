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



fn random_scene() -> HitableList {
    let mut rng = rand::thread_rng();
    let mut list: Vec<Box<dyn Hitable>> = vec![];
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian {
            attenuation: Vec3::new(0.5, 0.5, 0.5),
        },
    )));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen::<f64>();
            let center = Vec3::new(
                f64::from(a) + 0.9 * rng.gen::<f64>(),
                0.2,
                f64::from(b) + 0.9 * rng.gen::<f64>(),
            );
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Lambertian {
                            attenuation: Vec3::new(
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                                rng.gen::<f64>() * rng.gen::<f64>(),
                            ),
                        },
                    )));
                } else if choose_mat < 0.95 {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Metal {
                            attenuation: Vec3::new(
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                                0.5 * (1.0 + rng.gen::<f64>()),
                            ),
                            fuzzines: 0.5 * rng.gen::<f64>(),
                        },
                    )));
                } else {
                    list.push(Box::new(Sphere::new(
                        center,
                        0.2,
                        Material::Dielectric { refraction_idx: 1.5 },
                    )));
                }
            }
        }
    }
    list.push(Box::new(Sphere::new(
        Vec3::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric { refraction_idx: 1.5 },
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian {
            attenuation: Vec3::new(0.4, 0.2, 0.1),
        },
    )));
    list.push(Box::new(Sphere::new(
        Vec3::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal {
            attenuation: Vec3::new(0.7, 0.6, 0.5),
            fuzzines: 0.0,
        },
    )));
    HitableList::new(list)
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
    const IMAGE_WITH: i64 = 1920;
    const IMAGE_HEIGHT: i64 = (IMAGE_WITH as f64 / ASPECT_RATIO) as i64;
    const SAMPLES_PER_PIXEL: i64 = 600;
    const MAX_DEPHT: i64 = 50;

    let world = random_scene();


    let lookfrom = Vec3::new(13.0, 2.0, 3.0);
    let lookat = Vec3::new(0.0, 0.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 0.1;

    // Camera

    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        (IMAGE_WITH / IMAGE_HEIGHT) as f64,
        aperture,
        10.0,
    );

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
