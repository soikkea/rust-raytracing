use std::io::{self, Write};

use std::time::Instant;

use crate::color::write_color;
use crate::vec3::Color;

pub mod color;
pub mod vec3;

fn main() {
    // Image
    const IMAGE_WIDTH: u32 = 256;
    const IMAGE_HEIGHT: u32 = 256;

    // Render

    let start = Instant::now();

    println!("P3\n{IMAGE_WIDTH}  {IMAGE_HEIGHT}\n255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {j} ");
        io::stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
                0.25,
            );
            write_color(&mut io::stdout().lock(), pixel_color).unwrap();
        }
    }

    let duration = start.elapsed();
    eprint!("\nDone.\nTime elapsed while rendering: {:?}", duration);
}
