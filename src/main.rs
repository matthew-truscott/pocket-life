fn main() {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    print!("P3\n{0} {1}\n255\n", image_width, image_height);

    for j in 0..image_height {
        for i in 0..image_width {
            let r = i as f64 / (image_width as f64 - 1.0);
            let g = j as f64 / (image_height as f64 - 1.0);
            let b = 0.0f64;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            println!("{0} {1} {2}", ir, ig, ib);
        }
    }
}
