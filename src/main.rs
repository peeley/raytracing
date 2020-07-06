mod ray;
mod vec;

use ray::Ray;
use vec::{Color, Coordinate, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 384;
    let img_height = (img_width as f32 / aspect_ratio) as i32;
    println!("P3\n{} {}\n255", img_width, img_height);

    let viewport_height = 2.0;
    let viewport_width = viewport_height / aspect_ratio;
    let focal_length = 1.0;

    let origin: Vec3 = vec::Vec3::new(0.0, 0.0, 0.0);
    let horizontal: Coordinate = vec::Coordinate::new(viewport_width, 0.0, 0.0);
    let vertical: Coordinate = vec::Coordinate::new(0.0, viewport_height, 0.0);
    let lower_left_corner: Coordinate = origin
        - (horizontal / 2.0)
        - (vertical / 2.0)
        - vec::Coordinate::new(0.0, 0.0, focal_length);

    for y in (0..img_height).rev() {
        for x in 0..img_width {
            let u = x as f32 / (img_width as f32 - 1.0);
            let v = y as f32 / (img_height as f32 - 1.0);
            let ray = Ray::new(
                origin,
                lower_left_corner + (u * horizontal) + (v * vertical) - origin,
            );
            let ray_color: Color = ray.color();
            ray_color.print();
        }
    }
}
