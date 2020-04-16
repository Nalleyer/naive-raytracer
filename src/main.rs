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
    material::{Coloration, Material, SurfaceType, Texture, UniversalMaterial},
    Scene,
};

fn main() {
    test_can_render_scene();
}

fn test_can_render_scene() {
    let tex = image::open("tex.png").unwrap();
    let scene = Scene {
        width: 640,
        height: 480,
        fov: 90.0,
        items: vec![
            Box::new(Sphere {
                center: Point {
                    x: 0.0,
                    y: 0.5,
                    z: -3.0,
                },
                radius: 1.2,
                material: Box::new(UniversalMaterial {
                    color: Coloration::Color(Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    }),
                    albedo: 3.0,
                    index: 1.5,
                    transparency: 0.9,
                    reflectivity: 0.0,
                    emmit: 0.0,
                    is_light: false,
                })
            }),
            Box::new(Sphere {
                center: Point {
                    x: 4.0,
                    y: 2.0,
                    z: -7.5,
                },
                radius: 3.5,
                material: Box::new(UniversalMaterial {
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
                    albedo: 2.0,
                    index: 0.0,
                    transparency: 0.0,
                    reflectivity: 0.7,
                    emmit: 0.5,
                    is_light: false,
                }),
            }),
            Box::new(Sphere {
                center: Point {
                    x: -7.5,
                    y: 2.0,
                    z: -7.5,
                },
                radius: 5.0,
                material: Box::new(UniversalMaterial {
                    color: Coloration::Color(Color {
                        r: 0.0,
                        g: 0.0,
                        b: 1.0,
                    }),
                    albedo: 2.0,
                    index: 0.0,
                    transparency: 0.0,
                    reflectivity: 0.0,
                    emmit: 0.0,
                    is_light: false,
                }),
            }),
            // light 1
            Box::new(Sphere {
                center: Point {
                    x: 0.0,
                    y: 3.0,
                    z: -1.5,
                },
                radius: 1.0,
                material: Box::new(UniversalMaterial {
                    color: Coloration::Color(Color {
                        r: 1.0,
                        g: 1.0,
                        b: 0.0,
                    }),
                    albedo: 0.0,
                    index: 0.0,
                    transparency: 0.0,
                    reflectivity: 0.0,
                    emmit: 300.0,
                    is_light: true,
                }),
            }),
            // ground
            Box::new(Sphere {
                center: Point {
                    x: 0.0,
                    y: -1000.0,
                    z: -7.5,
                },
                radius: 995.0,
                material: Box::new(UniversalMaterial {
                    color: Coloration::Color(Color {
                        r: 1.0,
                        g: 1.0,
                        b: 1.0,
                    }),
                    albedo: 3.0,
                    index: 0.0,
                    transparency: 0.0,
                    reflectivity: 0.0,
                    emmit: 0.0,
                    is_light: false,
                }),
            }),
            /*
            Box::new(Plane {
                pos: Point {
                    x: 0.0,
                    y: -7.0,
                    z: -5.0,
                },
                normal: Vector3::new(0.0, -1.0, 0.0).normalize(),
                material: Box::new(UniversalMaterial {
                    color: Coloration::Texture(Texture {
                        image: tex.to_rgba(),
                        offset_x: 0.0,
                        offset_y: 0.0,
                        scale: 5.0,
                    }),
                    albedo: 0.5,
                    index: 0.0,
                    transparency: 0.0,
                    reflectivity: 0.4,
                    emmit: 0.0,
                }),
            }),
            Box::new(Plane {
                pos: Point {
                    x: 0.0,
                    y: 0.0,
                    z: -15.0,
                },
                normal: Vector3::new(0.0, 0.0, -1.0).normalize(),
                material: Box::new(UniversalMaterial {
                    color: Coloration::Texture(Texture {
                        image: tex.to_rgba(),
                        offset_x: 0.0,
                        offset_y: 0.0,
                        scale: 5.0,
                    }),
                    albedo: 0.5,
                    index: 0.0,
                    transparency: 0.0,
                    reflectivity: 0.4,
                    emmit: 0.5,
                }),
            }),
            */
        ],
    };

    let img = render(&scene).to_rgb();
    assert_eq!(scene.width, img.width());
    assert_eq!(scene.height, img.height());

    img.save("./test.png").unwrap();
}
