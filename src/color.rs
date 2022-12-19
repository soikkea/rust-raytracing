use crate::vec3::Color;

pub fn write_color(out: &mut impl std::io::Write, pixel_color: Color) -> std::io::Result<()> {
    let r = (255.999 * pixel_color.x()) as u32;
    let g = (255.999 * pixel_color.y()) as u32;
    let b = (255.999 * pixel_color.z()) as u32;
    write!(out, "{} {} {}\n", r, g, b)?;
    Ok(())
}
