use rayon::prelude::*;
use std::time::Instant;


fn calculate_point(x_pos: f64, y_pos: f64, max_iterations: u32) -> u32 {

    let mut x = 0.0;
    let mut y = 0.0;
    let mut iteration = 0;
    while f64::powf(x, 2.0) + f64::powf(y, 2.0) <= 4.0 && iteration < max_iterations {
        let x_temp = f64::powf(x, 2.0) - f64::powf(y, 2.0) + x_pos;
        y = 2.0*x*y + y_pos;
        x = x_temp;
        iteration+=1;
    }
    iteration
}

fn select_colour(iterations: u32) -> (u8, u8, u8) {
    if iterations > 16777215 {
        return (0, 0, 0)
    }

    let red = ((iterations >> 16) & 0xFF) as u8; // rbg
    let blue = ((iterations >> 8) & 0xFF) as u8;
    let green = ((iterations & 0xFF)) as u8;
    (red, green, blue)
}

fn generate_image_p(width: u32, height: u32, x1: f64, y1: f64, x2: f64, y2: f64, max_iterations: u32) {
    let mut img = image::RgbImage::new(width, height);

    let scale_x = (x2 - x1) / width as f64; 
    let scale_y = (y2 - y1) / height as f64;

    let buf = img.as_mut();

    buf.par_chunks_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = (i as u32) % width;
            let y = (i as u32) / height;
            let iterations = calculate_point(x as f64 * scale_x + x1, y as f64 * scale_y + y1, max_iterations);
            let (r, g, b) = select_colour(((16777216.0 / max_iterations as f64) * iterations as f64) as u32);
            pixel.copy_from_slice(&[r, g, b]);
        });

    img.save("fractal_p.png").unwrap();
}

fn main() {
    
    println!("Generating Mandelbrot Set - this may take a while...");

    let now = Instant::now();
    generate_image_p(1080, 1080, -2.0, -1.12, 0.47, 1.12, 1000);
    let elapsed = now.elapsed();

    println!("Done! Elapsed time: {:.2?}", elapsed);

}
