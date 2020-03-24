extern crate image;

use image::{DynamicImage, Rgba, RgbaImage, ImageBuffer};
mod math;
mod rendering;
mod scene;

use math::Point;
use rendering::{Ray, cast_ray, render};
use scene::{Color, Scene, Sphere};

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        items: vec![Box::new(Sphere {
            center: Point {
                x: 0.0,
                y: 0.0,
                z: -5.0,
            },
            radius: 1.0,
            color: Color {
                r: 0.4,
                g: 1.0,
                b: 0.4,
            },
        })],
    };

    let img = render(&scene).to_rgb();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    img.save("./test.png").unwrap();
}
