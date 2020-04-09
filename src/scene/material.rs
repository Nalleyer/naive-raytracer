use crate::color::Color;
use image::ImageBuffer;

#[derive(Clone)]
pub enum SurfaceType {
    Diffuse,
    Reflective { reflectivity: f32 },
}

#[derive(Clone)]
pub struct Material {
    pub color: Coloration,
    pub albedo: f32,
    pub surface: SurfaceType,
}

#[derive(Clone)]
pub enum Coloration {
    Color(Color),
    Texture(Texture),
}

#[derive(Clone)]
pub struct Texture {
    pub image: ImageBuffer<image::Rgba<u8>, std::vec::Vec<u8>>,
    pub offset_x: f32,
    pub offset_y: f32,
    pub scale: f32,
}

pub struct TextureCoords {
    pub u: f32,
    pub v: f32,
}

fn wrap(val: f32, bound: u32) -> u32 {
    let signed_bound = bound as i32;
    let float_coord = val * bound as f32;
    let wrapped_coord = (float_coord as i32) % signed_bound;
    if wrapped_coord < 0 {
        (wrapped_coord + signed_bound) as u32
    } else {
        wrapped_coord as u32
    }
}

impl Coloration {
    pub fn color(&self, texture_coords: &TextureCoords) -> Color {
        match self {
            Self::Color(c) => *c,
            Self::Texture(tex) => {
                let u = wrap(
                    (texture_coords.u + tex.offset_x) / tex.scale,
                    tex.image.width(),
                );
                let v = wrap(
                    (texture_coords.v + tex.offset_y) / tex.scale,
                    tex.image.height(),
                );
                Color::from_rgba8(tex.image.get_pixel(u, v).0)
            }
        }
    }
}
