use crate::ray::Ray;
use crate::vec::Coordinate;

pub struct Camera {
    origin: Coordinate,
    lower_left_corner: Coordinate,
    horizontal: Coordinate,
    vertical: Coordinate,
    lens_radius: f32,
    u: Coordinate,
    v: Coordinate,
    w: Coordinate,
}

impl Camera {
    pub fn new(
        look_from: Coordinate,
        look_at: Coordinate,
        vup: Coordinate,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_distance: f32,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vec();
        let u = Coordinate::cross(&vup, &w).unit_vec();
        let v = Coordinate::cross(&w, &u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner =
            origin - (horizontal / 2.0) - (vertical / 2.0) - (focus_distance * w);

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
            w,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let direction = self.lens_radius * Coordinate::random_in_unit_disk();
        let offset = self.u * direction.x + self.v * direction.y;
        Ray::new(
            self.origin + offset,
            self.lower_left_corner + (s * self.horizontal) + (t * self.vertical)
                - self.origin
                - offset,
        )
    }
}
