use std::sync::Arc;

mod vec3;

mod color;
use crate::camera::{Camera, CameraConfig};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::random::Random;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use color::Color;
use crate::bvh_node::BvhNode;

mod ray;
mod hittable;
mod sphere;
mod hittable_list;
mod utils;
mod interval;
mod camera;
mod random;
mod material;
mod aabb;
mod bvh_node;

fn main() {
    // scene version throughout the book
    // three_spheres();

    // final render
    many_spheres();
}

#[allow(dead_code)]
fn three_spheres() {
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

fn many_spheres() {
    // Rng
    let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);
    let mut rng = Random::from_os_or_seeded(rng_seed);

    // World
    let mut world = HittableList::default();

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(Point3::new(0.0,-1000.0,0.0), 1000.0, ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.f64();
            let center = Point3::new(a as f64 + 0.9*rng.f64(), 0.2, b as f64 + 0.9*rng.f64());

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random(&mut rng) * Color::random(&mut rng);
                    sphere_material = Arc::new(Lambertian::new(albedo));
                    let center2 = center + Vec3::new(0.0, rng.range_f64(0.0..0.5), 0.0);
                    world.add(Sphere::new_moving(center, center2, 0.2, sphere_material));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_range(&mut rng, 0.5..1.0);
                    let fuzz = rng.range_f64(0.0..0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                } else {
                    // glass
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Sphere::new(center, 0.2, sphere_material));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    // Use BVH
    world = HittableList::new(BvhNode::from_list(&world, &mut rng));

    // Camera
    let camera = Camera::new(CameraConfig {
        image_width: 1200,
        samples_per_pixel: 100,
        max_depth: 50,

        vfov: 20,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::zero(),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,
        ..Default::default()
    });

    camera.render(&world);
}
