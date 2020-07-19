#![feature(clamp)]

mod camera;
mod hittable;
mod material;
mod ray;
mod sphere;
mod vec;

use camera::Camera;
use hittable::HittableList;
use material::Material;
use rand::{thread_rng, Rng};
use sphere::Sphere;
use vec::{Color, Coordinate};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 480;
    let img_height = (img_width as f32 / aspect_ratio) as i32;
    println!("P3\n{} {}\n255", img_width, img_height);

    let samples_per_pix = 100;

    let look_from = Coordinate::new(13.0, 2.0, 3.0); // look_from
    let look_to = Coordinate::new(0.0, 0.0, 0.0); // look_at
    let camera = Camera::new(
        look_from,
        look_to,
        Coordinate::new(0.0, 1.0, 0.0), // vup
        20.0,                           // vfov
        aspect_ratio,                   // aspect_ratio
        0.1,                            // aperture
        10.0,                           // focus_distance
    );

    let scene = random_scene();

    let mut rng = thread_rng();
    for y in (0..img_height).rev() {
        eprintln!("{} scan lines left...", y);
        for x in 0..img_width {
            let mut color = Color::default();
            for _ in 0..samples_per_pix {
                let u = (x as f32 + rng.gen_range(0.0, 1.0)) / (img_width as f32 - 1.0);
                let v = (y as f32 + rng.gen_range(0.0, 1.0)) / (img_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);
                color += ray.color(&scene, 5);
            }
            color.print(samples_per_pix);
        }
    }
    eprintln!("Done!");
}

fn random_scene() -> HittableList<Sphere> {
    let mut rng = thread_rng();
    let mut world = HittableList::new(Sphere::new(
        Coordinate::new(0.0, -1000.0, 0.0),
        1000.0,
        Material::Lambertian(Color::new(0.5, 0.5, 0.5)),
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.gen_range(0.0, 1.0);
            let center = Coordinate::new(
                a as f32 + 0.9 * rng.gen_range(0.0, 1.0),
                0.2,
                b as f32 + 0.9 * rng.gen_range(0.0, 1.0),
            );
            if (center - Coordinate::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo =
                        Color::random(Some(0.0), Some(1.0)) * Color::random(Some(0.0), Some(1.0));
                    world.add(Sphere::new(center, 0.2, Material::Lambertian(albedo)))
                } else if choose_mat < 0.95 {
                    let albedo = Color::random(Some(0.5), Some(1.0));
                    let fuzziness = rng.gen_range(0.0, 0.5);
                    world.add(Sphere::new(center, 0.2, Material::Metal(albedo, fuzziness)));
                } else {
                    world.add(Sphere::new(center, 0.2, Material::Dielectric(1.5)))
                }
            }
        }
    }

    world.add(Sphere::new(
        Coordinate::new(0.0, 1.0, 0.0),
        1.0,
        Material::Dielectric(1.5),
    ));
    world.add(Sphere::new(
        Coordinate::new(-4.0, 1.0, 0.0),
        1.0,
        Material::Lambertian(Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new(
        Coordinate::new(4.0, 1.0, 0.0),
        1.0,
        Material::Metal(Color::new(0.7, 0.6, 0.5), 0.0),
    ));

    return world;
}
