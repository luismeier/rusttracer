mod vec3;
use crate::vec3::write_color;
use crate::vec3::Vec3;

fn main() {
    const IMAGE_WITH: i32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WITH, IMAGE_HEIGHT);
    
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WITH {
            let pixel_color = Vec3::new(i as f64/ IMAGE_WITH as f64, j as f64 / IMAGE_HEIGHT as f64, 0.25);
            write_color(pixel_color);


        }
    }
    eprint!("\nAll Done!");
}
