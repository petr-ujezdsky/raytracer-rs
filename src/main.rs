use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::Arc;

mod vec3;

mod color;
use crate::camera::{Camera, CameraConfig};
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use color::{write_color, Color};
use crate::material::{Dielectric, Lambertian, Metal};

mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;
mod random;
mod material;

fn main() {
    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left   = Arc::new(Dielectric::new(1.5));
    let material_bubble   = Arc::new(Dielectric::new(1.0 / 1.5));
    let material_right  = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut world = HittableList::default();
    world.add(Sphere::new(Point3::new( 0.0, -100.5, -1.0), 100.0, material_ground));
    world.add(Sphere::new(Point3::new( 0.0,    0.0, -1.2),   0.5, material_center));
    world.add(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.5, material_left));
    world.add(Sphere::new(Point3::new(-1.0,    0.0, -1.0),   0.4, material_bubble));
    world.add(Sphere::new(Point3::new( 1.0,    0.0, -1.0),   0.5, material_right));

    // Camera
    let camera = Camera::new(CameraConfig {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        vfov: 20,
        lookfrom: Point3::new(-2.0, 2.0, 1.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),
        defocus_angle: 10.0,
        focus_dist: 3.4,
        ..Default::default()
    });

    camera.render(&world);
}
