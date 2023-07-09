fn main() {
    const IMAGE_WITH: i32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    println!("P3\n{} {}\n255\n", IMAGE_WITH, IMAGE_HEIGHT);
    
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);

        for i in 0..IMAGE_WITH {
            let r = i as f32 / IMAGE_WITH as f32;
            let g = j as f32 / IMAGE_HEIGHT as f32;
            let b = 0.25;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;
            println!("{} {} {}", ir, ig, ib);
        }
    }
    eprint!("\nAll Done!");
}
