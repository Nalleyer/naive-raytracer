extern crate image;

mod math;
mod rendering;
mod scene;

use math::{Point, Vector3};
use rendering::{cast_ray, render, Ray};
use scene::{Color, Plane, Scene, Sphere};

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_can_render_scene() {
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        items: vec![
            Box::new(Sphere {
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
            }),
            Box::new(Sphere {
                center: Point {
                    x: 2.0,
                    y: 2.0,
                    z: -7.5,
                },
                radius: 3.5,
                color: Color {
                    r: 0.7,
                    g: 0.7,
                    b: 1.0,
                },
            }),
            Box::new(Plane {
                pos: Point {
                    x: 0.0,
                    y: -2.0,
                    z: -5.0,
                },
                normal: Vector3::new(0.0, -1.0, 0.0).normalize(),
                color: Color {
                    r: 1.0,
                    g: 0.2,
                    b: 0.2,
                },
            }),
        ],
    };

    let img = render(&scene).to_rgb();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    img.save("./test.png").unwrap();
}
