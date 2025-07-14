use rayon::prelude::*; // Bring Rayon traits into scope
use std::time::Instant;

fn select_char(iterations: u32, max_iterations: u32) -> String {
    let max_char = 100;

    let choice = ((max_char as f64 / max_iterations as f64) * iterations as f64) as i32;
    match choice {
        0 => " ".to_string(),
        1 => "'".to_string(),
        2 => ",".to_string(),
        3 => ".".to_string(),
        4 => "`".to_string(),
        5 => "\"".to_string(),
        6 => ":".to_string(),
        7 => ";".to_string(),
        8 => "|".to_string(),
        9 => "-".to_string(),
        10 => "=".to_string(),
        11 => "a".to_string(),
        12 => "b".to_string(),
        13 => "c".to_string(),
        14 => "d".to_string(),
        15 => "e".to_string(),
        16 => "f".to_string(),
        17 => "g".to_string(),
        18 => "h".to_string(),
        19 => "i".to_string(),
        20 => "j".to_string(),
        21 => "l".to_string(),
        22 => "m".to_string(),
        23 => "n".to_string(),
        24 => "o".to_string(),
        25 => "p".to_string(),
        26 => "q".to_string(),
        27 => "r".to_string(),
        28 => "s".to_string(),
        29 => "t".to_string(),
        30 => "u".to_string(),
        31 => "v".to_string(),
        32 => "w".to_string(),
        33 => "x".to_string(),
        34 => "y".to_string(),
        35 => "z".to_string(),
        36 => "A".to_string(),
        37 => "B".to_string(),
        38 => "C".to_string(),
        39 => "D".to_string(),
        40 => "E".to_string(),
        41 => "F".to_string(),
        42 => "G".to_string(),
        43 => "H".to_string(),
        44 => "I".to_string(),
        45 => "J".to_string(),
        46 => "K".to_string(),
        47 => "L".to_string(),
        48 => "M".to_string(),
        49 => "N".to_string(),
        50 => "O".to_string(),
        51 => "P".to_string(),
        52 => "Q".to_string(),
        53 => "R".to_string(),
        54 => "S".to_string(),
        55 => "T".to_string(),
        56 => "U".to_string(),
        57 => "V".to_string(),
        58 => "W".to_string(),
        59 => "X".to_string(),
        60 => "Y".to_string(),
        61 => "Z".to_string(),
        62 => "1".to_string(),
        63 => "2".to_string(),
        64 => "3".to_string(),
        65 => "4".to_string(),
        66 => "5".to_string(),
        67 => "6".to_string(),
        68 => "7".to_string(),
        69 => "8".to_string(),
        70 => "9".to_string(),
        71 => "0".to_string(),
        72 => "Ę".to_string(),
        73 => "Ó".to_string(),
        74 => "Ą".to_string(),
        75 => "Ś".to_string(),
        76 => "Ł".to_string(),
        77 => "Ż".to_string(),
        78 => "Ć".to_string(),
        79 => "Ń".to_string(),
        80 => "È".to_string(),
        81 => "Ô".to_string(),
        82 => "À".to_string(),
        83 => "Š".to_string(),
        84 => "Ź".to_string(),
        85 => "Ç".to_string(),
        86 => "É".to_string(),
        87 => "Ö".to_string(),
        88 => "Á".to_string(),
        89 => "Č".to_string(),
        90 => "Ê".to_string(),
        91 => "Ò".to_string(),
        92 => "Ë".to_string(),
        93 => "Õ".to_string(),
        94 => "Ė".to_string(),
        95 => "Œ".to_string(),
        96 => "Ø".to_string(),
        97 => "Ē".to_string(),
        98 => "Ō".to_string(),
        99 => ".".to_string(),
        _ => " ".to_string()
    }
}

fn calculate_point(x_pos: f64, y_pos: f64, max_iterations: u32) -> u32 {
    //let scaled_x = ((x_pos as f64 * scale_x) - (2.0+offset_x)) / zoom;
    //let scaled_y = ((y_pos as f64 * scale_y) - (1.12-offset_y))/ zoom;

    // outside the set?
    //if (scaled_x < -2.0 || scaled_x > 0.47) || (scaled_y < -1.12 || scaled_y > 1.12) {
    //    return 0;
    //}

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

//fn generate_text(width: u32, height: u32, offset_x: f64, offset_y: f64, zoom: f64) {
//    // internal params
//    let max_iterations = 99;
//    let scale_x = 2.47 / width as f64; 
//    let scale_y = 2.24 / height as f64;
//    for y in 0..height {
//        for x in 0..width {
//            let iterations = calculate_point(x, y, offset_x, offset_y, scale_x, scale_y, max_iterations);
//            print!("{}", select_char(iterations, max_iterations));
//        }
//        print!("\n");
//    }
//}

fn select_colour(iterations: u32) -> (u8, u8, u8) {
    if iterations > 16777215 {
        return (0, 0, 0)
    }

    let red = ((iterations >> 16) & 0xFF) as u8; // rbg
    let blue = ((iterations >> 8) & 0xFF) as u8;
    let green = ((iterations & 0xFF)) as u8;
    (red, green, blue)
}


fn generate_image_p(width: u32, height: u32, x1: f64, y1: f64, x2: f64, y2: f64) {
    let mut img = image::RgbImage::new(width, height);

    let max_iterations = 5000;
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
    
    //generate_text(64, 26, 0.0, 0.0, 1.0);
    
    let now = Instant::now();
    generate_image_p(1080, 1080, -2.0, -1.12, 0.47, 1.12);
    let elapsed = now.elapsed();
    println!("Elapsed time: {:.2?}", elapsed);

}
