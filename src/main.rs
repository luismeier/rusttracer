mod ray;
mod vec3;

use crate::ray::Ray;
use crate::vec3::write_color;
use crate::vec3::Vec3;

fn ray_color(ray: &Ray) -> Vec3 {
    let unit_dir = ray.direction().unit();
    let t = 0.5 * (unit_dir.y() + 1.0);
    return (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0);
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WITH: i64 = 400;
    const IMAGE_HEIGHT: i64 = (IMAGE_WITH as f64 / ASPECT_RATIO) as i64;

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

            let pixel_color = ray_color(&ray);
            write_color(pixel_color);
        }
    }
    eprint!("\nAll Done!");
}
