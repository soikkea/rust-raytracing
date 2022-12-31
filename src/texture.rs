use std::sync::Arc;

use crate::{
    perlin::Perlin,
    vec3::{Color, Point3},
};

pub trait Texture: Send + Sync {
    fn value(&self, u: f64, v: f64, point: &Point3) -> Color;
}

pub type TexturePtr = Arc<dyn Texture>;

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
        let even: TexturePtr = Arc::new(SolidColor::new(even_color));
        let odd: TexturePtr = Arc::new(SolidColor::new(odd_color));
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

pub struct NoiseTexture {
    pub noise: Perlin,
    pub scale: f64,
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: &Point3) -> Color {
        Color::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (self.scale * point.z() + 10.0 * self.noise.turbulence(point, 7)).sin())
    }
}

pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl ImageTexture {
    const BYTES_PER_PIXEL: u32 = 3;

    pub fn new(file_name: &str) -> ImageTexture {
        let image = image::open(file_name).map(|i| i.to_rgb8());

        let mut width = 0;
        let mut height = 0;
        let mut bytes_per_scanline = 0;

        let data;

        match image {
            Err(e) => {
                eprintln!("ERROR: Could not load texture file {}: {:?}", file_name, e);
                data = vec![];
            }
            Ok(image) => {
                width = image.width();
                height = image.height();
                data = image.into_raw();
                bytes_per_scanline = ImageTexture::BYTES_PER_PIXEL * width;
            }
        }

        ImageTexture {
            data,
            width,
            height,
            bytes_per_scanline,
        }
    }
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: &Point3) -> Color {
        // If we have no texture data, then return solid cyan as a debugging aid.
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0, 1] x [1, 0]
        let u = u.clamp(0.0, 1.0);
        let v = 1.0 - v.clamp(0.0, 1.0); // Flip V to image coordinates

        let mut i = (u * self.width as f64) as u32;
        let mut j = (v * self.height as f64) as u32;

        // Clamp integer mapping, since actual coordinates should be less than 1.0
        if i >= self.width {
            i = self.width - 1;
        }
        if j >= self.height {
            j = self.height - 1;
        }

        let color_scale = 1.0 / 255.0;
        let pixel_index =
            (j * self.bytes_per_scanline + i * ImageTexture::BYTES_PER_PIXEL) as usize;

        Color::new(
            color_scale * self.data[pixel_index] as f64,
            color_scale * self.data[pixel_index + 1] as f64,
            color_scale * self.data[pixel_index + 2] as f64,
        )
    }
}
