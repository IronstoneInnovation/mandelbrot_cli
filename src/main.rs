use rayon::prelude::*;
use std::time::Instant;
use enterpolation::{bspline::{BSpline}, Curve};
use palette::LinSrgb;


fn calculate_point(x_pos: f64, y_pos: f64, max_iterations: u32) -> u32 {
    // Calculates the critical number of iterations for a given point on the Mandelbrot Set
    // derived from https://en.wikipedia.org/wiki/Mandelbrot_set#Computer_drawings
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


fn generate_image_p(width: u32, height: u32, x1: f64, y1: f64, x2: f64, y2: f64, max_iterations: u32) {
    // Generates a Mandelbrot Set image and saves it as a PNG.

    // Set up colour pallete.
    // First create a colour curve...
    let bspline = match BSpline::builder()
        .clamped()             // the curve should be clamped (variation)
        .elements([
            LinSrgb::new(0.00, 0.00, 0.50),
            LinSrgb::new(0.00, 0.00, 1.00),
            LinSrgb::new(0.00, 1.00, 1.00),
            LinSrgb::new(1.0, 1.0, 1.0),
        ])
        .equidistant::<f64>() // knots should be evenly distributed
        .degree(3)            
        .domain(-2.0,2.0)     
        .constant::<4>()      // we need degree+1 space to interpolate
        .build() {
            Ok(curve) => curve,
            Err(error) => {
                panic!("Unexpected runtime error: {:?}", error);
            }
        };
    // ...then generate max_iterations number of colours along the curve 
    let colours: Vec<_> = bspline.take(max_iterations as usize + 1).collect();

    // x, y scale factors
    let scale_x = (x2 - x1) / width as f64; 
    let scale_y = (y2 - y1) / height as f64;

    // Create a new blank image and a buffer for parallel processing
    let mut img = image::RgbImage::new(width, height);
    let buf = img.as_mut();

    // Calculate colours and update pixels in parallel
    buf.par_chunks_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            // determine x, y from array index
            let x = (i as u32) % width;
            let y = (i as u32) / height;
            // calculate iterations for this point
            let iterations = calculate_point(x as f64 * scale_x + x1, y as f64 * scale_y + y1, max_iterations);
            // pick colour
            let mut rgb = colours[iterations as usize];
            // black saturation option
            if iterations >= max_iterations {
                rgb = LinSrgb::new(0.0, 0.0, 0.0);
            }
            // convert to u8 RGB and update pixel
            let src = [(rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8];
            pixel.copy_from_slice(&src);
        });

    // Done - save generated image
    img.save("fractal_p.png").unwrap();
}

fn main() {
    let magnification = 2.0;
    let x_offset = 0.0;
    let y_offset = 0.0;

    println!("Generating Mandelbrot Set - this may take a while...");

    
    let x_min = -2.0;
    let y_min = -1.12;
    let x_max = 0.47;
    let y_max = 1.12;

    // Calculate x, y range
    let x_length = x_max - x_min;
    let y_length = y_max - y_min;
    let x_midpoint = x_min + (x_length / 2.0) + x_offset;
    let y_midpoint = y_min + (y_length / 2.0) + y_offset;

    let x1 = x_midpoint - (x_length / (2.0 * magnification));
    let y1 = y_midpoint - (y_length / (2.0 * magnification));
    let x2 = x_midpoint + (x_length / (2.0 * magnification));
    let y2 = y_midpoint + (y_length / (2.0 * magnification));
    println!("x1, y1 = {:?}; x2, y2 = {:?}", (x1, y1), (x2, y2));

    // Generate image and measure elapsed time
    let now = Instant::now();
    generate_image_p(1080, 1080, x1, y1, x2, y2, 1000);
    let elapsed = now.elapsed();

    println!("Done! Elapsed time: {:.2?}", elapsed);

}
