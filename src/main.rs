mod hitable;
mod hitable_list;
mod ray;
mod sphere;
mod vec3;

use crate::hitable_list::HitableList;
use crate::sphere::Sphere;
use crate::vec3::write_color;
use crate::vec3::Vec3;
use hitable::Hitable;
use ray::Ray;

use Vec3 as Color;

fn ray_color(ray: &Ray, world: &dyn Hitable) -> Color {
    match world.hit(ray, 0.0, std::f64::INFINITY) {
        Some(hit_record) => 0.5 * (hit_record.normal + Color::new(1.0, 1.0, 1.0)),
        None => {
            let unit_direction = ray.direction().unit();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WITH: i64 = 800;
    const IMAGE_HEIGHT: i64 = (IMAGE_WITH as f64 / ASPECT_RATIO) as i64;

    // World
    let world = HitableList::new(vec![
        Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
    ]);

    // Camera
    let viewport_height: f64 = 2.0;
    let viewport_with: f64 = ASPECT_RATIO * viewport_height;
    let focal_length: f64 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_with, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0., 0.0, focal_length);

    println!("P3\n{} {}\n255\n", IMAGE_WITH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WITH {
            let u = i as f64 / IMAGE_WITH as f64;
            let v = j as f64 / IMAGE_HEIGHT as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_color = ray_color(&ray, &world);
            write_color(pixel_color);
        }
    }
    eprint!("\nAll Done!");
}
