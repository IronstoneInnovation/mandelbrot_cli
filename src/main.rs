use rayon::prelude::*;
use std::time::Instant;
use enterpolation::{bspline::{BSpline}, Curve};
use palette::LinSrgb;
use image::ImageBuffer;
use clap::Parser;

/// Calculates the critical number of iterations for a given point on the Mandelbrot Set.
pub fn calculate_point(x_pos: f64, y_pos: f64, max_iterations: u32) -> u32 {
    // Derived from https://en.wikipedia.org/wiki/Mandelbrot_set#Computer_drawings
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

/// Generate an image of the Mandelbrot Set.
/// 
/// # Arguments
/// - **width, height**: Width and height of the image in pixels
/// - **x1**, **y1**: Bottom-left corner of the rectangle to draw
/// - **x2**, **y2**: Top-right corner of the rectangle to draw
/// - **max_iterations**: The maximum number of iterations per point
/// 
/// # Recommended values
/// - width and height can be anything you want, 1080x1080 is recommended
/// - x1, y1, x2, y2 should be -2.0, -1.12, 0.47, 1.12 to draw the entire Set
/// - max_iterations: 1000 provides a good compromise between colour depth and performance
pub fn generate_image(width: u32, height: u32, x1: f64, y1: f64, x2: f64, y2: f64, max_iterations: u32) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    // Set up colour pallete.
    // First create a colour curve...
    let bspline = match BSpline::builder()
        .clamped()             // the curve should be clamped (variation)
        .elements([
            LinSrgb::new(0.00, 0.00, 0.15),
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

    // Calculate colours, update pixels in parallel and return image
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
    img
}


#[derive(Parser, Default, Debug)]
struct Cli {
    #[clap(short, long, default_value_t = 1080)]
    size: u32,
    #[clap(short, long, default_value_t = 0.0)]
    x_offset: f64,
    #[clap(short, long, default_value_t = 0.0)]
    y_offset: f64,
    #[clap(short, long, default_value_t = 1.0)]
    magnification: f64,
    #[clap(short, long, default_value_t = 100)]
    iterations: u32,
    #[clap(short, long, default_value = "out.png" )]
    output_path: std::path::PathBuf,
}

/// Calculate the x, y coords of a rectanglular section of the Mandelbrot Set for a given
/// x, y offset and magnification.
/// 
/// The rectangle is centered over the midpoint of the complete Set (as opposed to origin, 
/// which is off-center), unless x_offset and/or y_offset are non-zero.  In this way you
/// can recentre the image over a selected point on the Set and magnify to reveal more detail.
/// 
/// # Arguments
/// - **x_offset**, **y_offset**: Coords of the offset; any value you like but remember the
/// main action is between (-2.0, -1.12) and (0.47, 1.12)
/// - **magnification**: 1.0 for no magnification, otherwise any value > 1.0 to zoom in
pub fn calculate_rectangle(x_offset: f64, y_offset: f64, magnification: f64) -> (f64, f64, f64, f64) {
    let x_min = -2.0;
    let y_min = -1.12;
    let x_max = 0.47;
    let y_max = 1.12;
    let x_length = x_max - x_min;
    let y_length = y_max - y_min;
    let x_midpoint = x_min + (x_length / 2.0) + x_offset;
    let y_midpoint = y_min + (y_length / 2.0) - y_offset;  // - because images are upside down
    let x1 = x_midpoint - (x_length / (2.0 * magnification));
    let y1 = y_midpoint - (y_length / (2.0 * magnification));
    let x2 = x_midpoint + (x_length / (2.0 * magnification));
    let y2 = y_midpoint + (y_length / (2.0 * magnification));
    (x1, y1, x2, y2)
}

fn main() {

    // Extract CLI args
    let args = Cli::parse();
    let magnification = args.magnification;
    let x_offset = args.x_offset;
    let y_offset = args.y_offset;
    let image_size = args.size;
    let max_iterations = args.iterations;

    // Get x, y coords of rectangle to draw
    let (x1, y1, x2, y2) = calculate_rectangle(x_offset, y_offset, magnification);

    // Generate image and measure elapsed time
    println!("Generating Mandelbrot Set - this may take a while...");
    let now = Instant::now();
    let img = generate_image(image_size, image_size, x1, y1, x2, y2, max_iterations);
    let elapsed = now.elapsed();
    println!("Done! Elapsed time: {:.2?}", elapsed);

    img.save(args.output_path).unwrap();

}
