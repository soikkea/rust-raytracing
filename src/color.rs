use crate::vec3::Color;

pub fn write_color(
    out: &mut impl std::io::Write,
    pixel_color: Color,
    samples_per_pixel: u32,
) -> std::io::Result<()> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    let scale = 1.0 / (samples_per_pixel as f64);
    r *= scale;
    g *= scale;
    b *= scale;

    let r_out = (256.0 * r.clamp(0.0, 0.999)) as u32;
    let g_out = (256.0 * g.clamp(0.0, 0.999)) as u32;
    let b_out = (256.0 * b.clamp(0.0, 0.999)) as u32;

    write!(out, "{} {} {}\n", r_out, g_out, b_out)?;
    Ok(())
}
