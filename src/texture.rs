use std::sync::Arc;

use crate::vec3::{Color, Point3};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color;
}

pub type TexturePtr = Arc<Box<dyn Texture>>;

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color_value: color }
    }

    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        let color = Color::new(r, g, b);
        SolidColor::new(color)
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _point: &Point3) -> Color {
        self.color_value
    }
}

pub struct CheckerTexture {
    pub odd: TexturePtr,
    pub even: TexturePtr,
}

impl CheckerTexture {
    pub fn new(even: &TexturePtr, odd: &TexturePtr) -> CheckerTexture {
        let even = Arc::clone(even);
        let odd = Arc::clone(odd);
        CheckerTexture { odd, even }
    }

    pub fn new_from_colors(even_color: Color, odd_color: Color) -> CheckerTexture {
        let even: TexturePtr = Arc::new(Box::new(SolidColor::new(even_color)));
        let odd: TexturePtr = Arc::new(Box::new(SolidColor::new(odd_color)));
        CheckerTexture::new(&even, &odd)
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color {
        let sines = (10.0 * point.x()).sin() * (10.0 * point.y()).sin() * (10.0 * point.z()).sin();
        if sines < 0.0 {
            self.odd.value(u, v, point)
        } else {
            self.even.value(u, v, point)
        }
    }
}
