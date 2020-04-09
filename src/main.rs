extern crate image;

mod color;
mod math;
mod rendering;
mod scene;

use color::Color;
use math::{Point, Vector3};
use rendering::render;
use scene::{
    item::{Plane, Sphere},
    light::{DirectionalLight, SphericalLight},
    material::{Coloration, Material, Texture, SurfaceType},
    Scene,
};

fn main() {
    test_can_render_scene();
}

fn test_can_render_scene() {
    let tex = image::open("tex.png").unwrap();
    let scene = Scene {
        width: 800,
        height: 600,
        fov: 90.0,
        items: vec![
            Box::new(Sphere {
                center: Point {
                    x: 0.0,
                    y: 2.0,
                    z: -3.0,
                },
                radius: 1.0,
                material: Material {
                    color: Coloration::Color(Color {
                        r: 0.0,
                        g: 1.0,
                        b: 0.0,
                    }),
                    albedo: 0.5,
                    surface: SurfaceType::Diffuse,
                },
            }),
            Box::new(Sphere {
                center: Point {
                    x: 2.0,
                    y: 2.0,
                    z: -7.5,
                },
                radius: 3.5,
                material: Material {
                    color: Coloration::Texture(Texture {
                        image: tex.to_rgba(),
                        offset_x: 0.0,
                        offset_y: 0.0,
                        scale: 0.1,
                    }),
                    /*
                    color: Coloration::Color(Color{
                        r: 1.0,
                        g: 0.0,
                        b: 0.0,
                    }),
                    */
                    albedo: 0.99,
                    surface: SurfaceType::Reflective { reflectivity: 0.7 },
                },
            }),
            Box::new(Sphere {
                center: Point {
                    x: -7.5,
                    y: 2.0,
                    z: -7.5,
                },
                radius: 5.0,
                material: Material {
                    color: Coloration::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,
                    }),
                    albedo: 2.0,
                    surface: SurfaceType::Diffuse,
                },
            }),
            Box::new(Plane {
                pos: Point {
                    x: 0.0,
                    y: -2.0,
                    z: -5.0,
                },
                normal: Vector3::new(0.0, -1.0, 0.0).normalize(),
                material: Material {
                    color: Coloration::Texture(Texture {
                        image: tex.to_rgba(),
                        offset_x: 0.0,
                        offset_y: 0.0,
                        scale: 5.0,
                    }),
                    albedo: 0.5,
                    surface: SurfaceType::Reflective { reflectivity: 0.4 },
                },
            }),
        ],
        lights: vec![
            Box::new(DirectionalLight {
                direction: Vector3::new(-0.5, -1.0, 0.1).normalize(),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                },
                intensity: 2.0,
            }),
            Box::new(SphericalLight {
                position: Point::new(3.0, 2.0, -3.0),
                color: Color {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                },
                intensity: 255.0,
            }),
        ],
    };

    let img = render(&scene).to_rgb();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    img.save("./test.png").unwrap();
}
