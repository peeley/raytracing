#![feature(clamp)]

mod hittable;
mod ray;
mod sphere;
mod vec;
mod camera;

use hittable::HittableList;
use sphere::Sphere;
use vec::{Color, Coordinate};
use camera::Camera;
use rand::{thread_rng, Rng};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 384;
    let img_height = (img_width as f32 / aspect_ratio) as i32;
    println!("P3\n{} {}\n255", img_width, img_height);

    let samples_per_pix = 100;

    let camera = Camera::new();

    let mut geometry = HittableList::new(Sphere::new(Coordinate::new(0.0, 0.0, -1.0), 0.5));
    geometry.add(Sphere::new(Coordinate::new(0.0, -100.5, -1.0), 100.0));

    let mut rng = thread_rng();
    for y in (0..img_height).rev() {
        for x in 0..img_width {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pix {
                let u = (x as f32 + rng.gen_range(0.0, 1.0)) / (img_width as f32 - 1.0);
                let v = (y as f32 + rng.gen_range(0.0, 1.0)) / (img_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);
                color += ray.color(&geometry);
            }
            color.print(samples_per_pix);
        }
    }
}
