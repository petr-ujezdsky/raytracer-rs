use std::sync::Arc;

mod vec3;

mod color;
use crate::camera::{Camera, CameraConfig};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::random::Random;
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use color::Color;
use crate::bvh_node::BvhNode;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::quad::Quad;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};

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
mod texture;
mod perlin;
mod quad;
mod constant_medium;
mod frame_buffer;
mod tile_manager;
mod onb;
mod pdf;

fn main() {
    match 7 {
        0 => three_spheres(),
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 1000, 40),
        99 => final_scene(400, 250, 4),
        _ => panic!("invalid scene number"),
    }
}

fn three_spheres() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);

    // Materials
    let material_ground = Arc::new(Lambertian::from_color(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::from_color(Color::new(0.1, 0.2, 0.5)));
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
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20,
        lookfrom: Point3::new(-2.0, 2.0, 1.0),
        lookat: Point3::new(0.0, 0.0, -1.0),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 10.0,
        focus_dist: 3.4,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}

fn bouncing_spheres() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);
    let mut rng = Random::from_os_or_seeded(rng_seed);

    // World
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    let ground_material = Arc::new(Lambertian::new(checker));
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
                    sphere_material = Arc::new(Lambertian::from_color(albedo));
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

    let material2 = Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
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
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::zero(),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}

fn checkered_spheres() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);

    // World
    let mut world = HittableList::default();

    let checker = Arc::new(CheckerTexture::from_colors(0.32, Color::new(0.2, 0.3, 0.1), Color::new(0.9, 0.9, 0.9)));

    world.add(Sphere::new(Point3::new(0.0,-10.0,0.0), 10.0, Arc::new(Lambertian::new(checker.clone()))));
    world.add(Sphere::new(Point3::new(0.0,10.0,0.0), 10.0, Arc::new(Lambertian::new(checker))));


    // Use BVH
    // world = HittableList::new(BvhNode::from_list(&world, &mut rng));

    // Camera
    let camera = Camera::new(CameraConfig {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::zero(),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        // focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}

fn earth() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);

    // World
    let mut world = HittableList::default();

    let earth_texture = Arc::new(ImageTexture::new(&"assets/earthmap.jpg".to_string()));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface);
    world.add(globe);


    // Use BVH
    // world = HittableList::new(BvhNode::from_list(&world, &mut rng));

    // Camera
    let camera = Camera::new(CameraConfig {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20,
        lookfrom: Point3::new(0.0, 0.0, 12.0),
        lookat: Point3::zero(),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        // focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}

fn perlin_spheres() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);
    let mut rng = Random::from_os_or_seeded(rng_seed);

    // World
    let mut world = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0, &mut rng));

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(pertext.clone()))));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::new(pertext))));

    // Camera
    let camera = Camera::new(CameraConfig {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 20,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::zero(),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        // focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}

fn quads() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);

    let mut world = HittableList::default();

    // Materials
    let left_red = Arc::new(Lambertian::from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from_color(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Quad::new(Point3::new(-3.0, -2.0, 5.0), Vec3::new(0.0, 0.0, -4.0), Vec3::new(0.0, 4.0, 0.0), left_red));
    world.add(Quad::new(Point3::new(-2.0, -2.0, 0.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 4.0, 0.0), back_green));
    world.add(Quad::new(Point3::new(3.0, -2.0, 1.0), Vec3::new(0.0, 0.0, 4.0), Vec3::new(0.0, 4.0, 0.0), right_blue));
    world.add(Quad::new(Point3::new(-2.0, 3.0, 1.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 4.0), upper_orange));
    world.add(Quad::new(Point3::new(-2.0, -3.0, 5.0), Vec3::new(4.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -4.0), lower_teal));

    // Camera
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 1.0,
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::new(0.7, 0.8, 1.0),

        vfov: 80,
        lookfrom: Point3::new(0.0, 0.0, 9.0),
        lookat: Point3::zero(),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        // focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}

fn simple_light() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);
    let mut rng = Random::from_os_or_seeded(rng_seed);

    // World
    let mut world = HittableList::default();

    let pertext = Arc::new(NoiseTexture::new(4.0, &mut rng));

    world.add(Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, Arc::new(Lambertian::new(pertext.clone()))));
    world.add(Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, Arc::new(Lambertian::new(pertext))));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));
    world.add(Sphere::new(Point3::new(0.0, 7.0, 0.0), 2.0, difflight.clone()));
    world.add(Quad::new(Point3::new(3.0, 1.0, -2.0), Vec3::new(2.0, 0.0, 0.0), Vec3::new(0.0, 2.0, 0.0), difflight));

    // Camera
    let camera = Camera::new(CameraConfig {
        image_width: 400,
        samples_per_pixel: 100,
        max_depth: 50,
        background: Color::zero(),

        vfov: 20,
        lookfrom: Point3::new(26.0, 3.0, 6.0),
        lookat: Point3::new(0.0, 2.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        // focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}

fn cornell_box() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);

    // World
    let mut world = HittableList::default();

    // Materials
    let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    // cornell sides
    world.add(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green));
    world.add(Quad::new(Point3::zero(), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red));
    world.add(Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light.clone()));
    world.add(Quad::new(Point3::zero(), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone()));
    world.add(Quad::new(Point3::new(555.0, 555.0, 555.0), Vec3::new(-555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -555.0), white.clone()));
    world.add(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone()));

    // boxes inside the cornell box
    let mut box1: Arc<dyn Hittable> = Arc::new(Quad::create_box(Point3::zero(), Point3::new(165.0, 330.0, 165.0), white.clone()));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add_arc(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Quad::create_box(Point3::zero(), Point3::new(165.0, 165.0, 165.0), white.clone()));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add_arc(box2);

    // Light sources
    // let empty_material = Arc::new();
    let lights = Quad::new(Point3::new(343.0, 554.0, 332.0), Vec3::new(-130.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -105.0), light.clone());

    // Camera
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 1024,
        max_depth: 50,
        background: Color::zero(),

        vfov: 40,
        lookfrom: Point3::new(278.0, 278.0, -800.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        // focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render2(&world, &lights);
}

fn cornell_smoke() {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);

    // World
    let mut world = HittableList::default();

    // Materials
    let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    // cornell sides
    world.add(Quad::new(Point3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), green));
    world.add(Quad::new(Point3::zero(), Vec3::new(0.0, 555.0, 0.0), Vec3::new(0.0, 0.0, 555.0), red));
    world.add(Quad::new(Point3::new(113.0,554.0,127.0), Vec3::new(330.0,0.0,0.0), Vec3::new(0.0, 0.0, 305.0), light));
    world.add(Quad::new(Point3::new(0.0, 555.0, 0.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone()));
    world.add(Quad::new(Point3::zero(), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 555.0), white.clone()));
    world.add(Quad::new(Point3::new(0.0, 0.0, 555.0), Vec3::new(555.0, 0.0, 0.0), Vec3::new(0.0, 555.0, 0.0), white.clone()));

    // boxes inside the cornell box
    let mut box1: Arc<dyn Hittable> = Arc::new(Quad::create_box(Point3::zero(), Point3::new(165.0, 330.0, 165.0), white.clone()));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    box1 = Arc::new(ConstantMedium::from_color(box1, 0.01, Color::zero()));
    world.add_arc(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Quad::create_box(Point3::zero(), Point3::new(165.0, 165.0, 165.0), white.clone()));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    box2 = Arc::new(ConstantMedium::from_color(box2, 0.01, Color::new(1.0, 1.0, 1.0)));
    world.add_arc(box2);

    // Camera
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 1.0,
        image_width: 600,
        samples_per_pixel: 200,
        max_depth: 50,
        background: Color::zero(),

        vfov: 40,
        lookfrom: Point3::new(278.0, 278.0, -800.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        // focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: u32) {
    // Rng
    // let rng_seed: Option<u64> = None;
    let rng_seed = Some(12487324);
    let mut rng = Random::from_os_or_seeded(rng_seed);

    // Boxes
    let mut boxes1 = HittableList::default();
    let ground = Arc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.range(1.0..101.0);
            let z1 = z0 + w;

            boxes1.add(Quad::create_box(Point3::new(x0, y0, z0), Point3::new(x1, y1, z1), ground.clone()));
        }
    }

    // World
    let mut world = HittableList::default();

    // Add boxes ground
    world.add(boxes1);

    // Light
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Quad::new(Point3::new(123.0, 554.0, 147.0), Vec3::new(300.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 265.0), light.clone()));

    // Moving sphere
    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::from_color(Color::new(0.7, 0.3, 0.1)));
    world.add(Sphere::new_moving(center1, center2, 50.0, sphere_material));

    // Other spheres
    world.add(Sphere::new(Point3::new(260.0, 150.0, 45.0), 50.0, Arc::new(Dielectric::new(1.5))));
    world.add(Sphere::new(Point3::new(0.0, 150.0, 145.0), 50.0, Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0))));

    let boundary1 = Arc::new(Sphere::new(Point3::new(360.0, 150.0, 145.0), 70.0, Arc::new(Dielectric::new(1.5))));
    world.add_arc(boundary1.clone());
    world.add(ConstantMedium::from_color(boundary1.clone(), 0.2, Color::new(0.2, 0.4, 0.9)));

    let boundary2 = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 5000.0, Arc::new(Dielectric::new(1.5))));
    world.add(ConstantMedium::from_color(boundary2.clone(), 0.0001, Color::new(1.0, 1.0, 1.0)));

    // Earth
    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(&"assets/earthmap.jpg".to_string()))));
    world.add(Sphere::new(Point3::new(400.0, 200.0, 400.0), 100.0, emat));
    let pertext = Arc::new(NoiseTexture::new(0.2, &mut rng));
    world.add(Sphere::new(Point3::new(220.0, 280.0, 300.0), 80.0, Arc::new(Lambertian::new(pertext))));

    // Boxes 2
    let mut boxes2 = HittableList::default();
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Sphere::new(Point3::random_range(&mut rng, 0.0..165.0), 10.0, white.clone()));
    }

    world.add(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::from_list(&boxes2, &mut rng)), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0)
    ));

    // Camera
    let camera = Camera::new(CameraConfig {
        aspect_ratio: 1.0,
        image_width,
        samples_per_pixel,
        max_depth,
        background: Color::zero(),

        vfov: 40,
        lookfrom: Point3::new(478.0, 278.0, -600.0),
        lookat: Point3::new(278.0, 278.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.0,
        // focus_dist: 10.0,

        rng_seed,
        ..Default::default()
    });

    camera.render(&world);
}
