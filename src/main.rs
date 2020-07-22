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
use std::sync::{Arc, Mutex};
use std::thread;
use vec::{Color, Coordinate};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 960;
    let img_height = (img_width as f32 / aspect_ratio) as i32;
    println!("P3\n{} {}\n255", img_width, img_height);

    let samples_per_pix = 100;

    let look_from = Coordinate::new(13.0, 2.0, 3.0); // look_from
    let look_to = Coordinate::new(0.0, 0.0, 0.0); // look_at
    let camera = Arc::new(Camera::new(
        look_from,
        look_to,
        Coordinate::new(0.0, 1.0, 0.0), // vup
        20.0,                           // vfov
        aspect_ratio,                   // aspect_ratio
        0.1,                            // aperture
        10.0,                           // focus_distance
    ));

    let scene = Arc::new(random_scene());

    let mut thread_handles = vec![];
    let num_cores = 4;
    let block_size = img_height / num_cores;
    let num_pixels = (img_height * img_width) as usize;
    let colors: Arc<Mutex<std::vec::Vec<String>>> =
        Arc::new(Mutex::new(vec![String::new(); num_pixels]));

    for block_num in 0..num_cores {
        let block_start = block_num * block_size;
        let block_end = block_start + block_size;
        eprintln!("scanning lines {} through {}", block_start, block_end);

        let cam_clone = Arc::clone(&camera);
        let scene_clone = Arc::clone(&scene);
        let colors_clone = Arc::clone(&colors);
        let thread_handle = thread::spawn(move || {
            for y in block_start..block_end {
                eprintln!("{} scan lines left...", y);
                let mut rng = thread_rng();
                for x in 0..img_width {
                    let mut color = Color::default();

                    for _ in 0..samples_per_pix {
                        let u = (x as f32 + rng.gen_range(0.0, 1.0)) / (img_width as f32 - 1.0);
                        let v = (y as f32 + rng.gen_range(0.0, 1.0)) / (img_height as f32 - 1.0);
                        let ray = cam_clone.get_ray(u, v);
                        color += ray.color(&scene_clone, 5);
                    }
                    let mut color_lock = (*colors_clone).lock().unwrap();
                    let pix_idx = (y * img_width + x) as usize;
                    (*color_lock)[pix_idx] = color.to_string(samples_per_pix);
                }
            }
        });
        thread_handles.push(thread_handle);
    }
    for handle in thread_handles {
        handle.join().unwrap();
    }
    let colors_clone = Arc::clone(&colors);
    let color_lock = (*colors_clone).lock().unwrap();
    for color in &*color_lock {
        println!("{}", color);
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
