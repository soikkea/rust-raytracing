use crate::vec3::{self, unit_vector, Color, Vec3};

pub fn write_color(
    out: &mut impl std::io::Write,
    pixel_color: vec3::Color,
    samples_per_pixel: u32,
) -> std::io::Result<()> {
    let (r_out, g_out, b_out) = color_to_rgb(pixel_color, samples_per_pixel);

    writeln!(out, "{} {} {}", r_out, g_out, b_out)?;
    Ok(())
}

fn color_to_rgb(pixel_color: vec3::Color, samples_per_pixel: u32) -> (u8, u8, u8) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    // Divide the color by the number of samples and gamma-correct for gamma=2.0.
    let scale = 1.0 / (samples_per_pixel as f64);
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    let r_out = (256.0 * r.clamp(0.0, 0.999)) as u8;
    let g_out = (256.0 * g.clamp(0.0, 0.999)) as u8;
    let b_out = (256.0 * b.clamp(0.0, 0.999)) as u8;
    (r_out, g_out, b_out)
}

pub fn color_to_pixel(pixel_color: vec3::Color, samples_per_pixel: u32) -> image::Rgb<u8> {
    let (r, g, b) = color_to_rgb(pixel_color, samples_per_pixel);

    image::Rgb([r, g, b])
}

#[derive(Clone, Copy)]
pub enum Background {
    Solid(Color),
    Gradient(Color, Color),
}

impl Background {
    pub fn background_color(&self, direction: &Vec3) -> Color {
        match self {
            Background::Solid(color) => *color,
            Background::Gradient(start_color, end_color) => {
                let unit_direction = unit_vector(direction);
                let t = 0.5 * (unit_direction.y() + 1.0);
                (1.0 - t) * end_color + t * start_color
            }
        }
    }
}
