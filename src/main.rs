use rayon::prelude::*;
use std::time::Instant;
use enterpolation::{bspline::{BSpline}, Curve};
use palette::LinSrgb;


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


fn generate_image_p(width: u32, height: u32, x1: f64, y1: f64, x2: f64, y2: f64, max_iterations: u32) {
    let mut img = image::RgbImage::new(width, height);

    let scale_x = (x2 - x1) / width as f64; 
    let scale_y = (y2 - y1) / height as f64;

    let bspline = match BSpline::builder()
        .clamped()             // the curve should be clamped (variation)
        .elements([
            LinSrgb::new(0.00, 0.00, 0.00),
            LinSrgb::new(0.00, 0.00, 0.95),
            LinSrgb::new(0.00, 0.95, 0.95),
            LinSrgb::new(0.95, 0.95, 0.95),
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

    let taken_colors: Vec<_> = bspline.take(max_iterations as usize + 1).collect();

    let buf = img.as_mut();

    buf.par_chunks_mut(3)
        .enumerate()
        .for_each(|(i, pixel)| {
            let x = (i as u32) % width;
            let y = (i as u32) / height;
            let iterations = calculate_point(x as f64 * scale_x + x1, y as f64 * scale_y + y1, max_iterations);

            
            let mut rgb = taken_colors[iterations as usize];

            // black saturation option
            if iterations >= max_iterations {
                rgb = LinSrgb::new(0.0, 0.0, 0.0);
            }

            let src = [(rgb.red * 255.0) as u8, (rgb.green * 255.0) as u8, (rgb.blue * 255.0) as u8];
            pixel.copy_from_slice(&src);
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
