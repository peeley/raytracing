use crate::ray::Ray;
use crate::vec::Coordinate;

pub struct Camera {
    origin: Coordinate,
    lower_left_corner: Coordinate,
    horizontal: Coordinate,
    vertical: Coordinate,
}

impl Camera {
    pub fn new(look_from: Coordinate, look_at: Coordinate, vup: Coordinate, vfov: f32, aspect_ratio: f32) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vec();
        let u = Coordinate::cross(&vup, &w).unit_vec();
        let v = Coordinate::cross(&w, &u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin
            - (horizontal / 2.0)
            - (vertical / 2.0)
            - w;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + (u * self.horizontal) + (v * self.vertical) - self.origin,
        )
    }
}
