use crate::vec::{Color, Coordinate, Vec3};

pub struct Ray {
    origin: Coordinate,
    direction: Coordinate,
}

impl Ray {
    pub fn at(&self, t: f32) -> Coordinate {
        return self.origin + (self.direction * t);
    }
    pub fn color(&self) -> Color {
        if self.intersects_sphere(Coordinate::new(0.0, 0.0, -1.0), 0.5) {
            return Color::new(1.0, 0.0, 0.0);
        }
        let unit_dir = self.direction.unit_vec();
        let t = 0.5 * (unit_dir.y + 1.0);
        return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
    }
    pub fn intersects_sphere(&self, center: Coordinate, radius: f32) -> bool {
        let oc = self.origin - center;
        let a = Vec3::dot(&self.direction, &self.direction);
        let b = 2.0 * Vec3::dot(&oc, &self.direction);
        let c = Vec3::dot(&oc, &oc) - (radius * radius);
        let discriminant = (b * b) - (4.0 * a * c);
        return discriminant > 0.0;
    }
    pub fn new(origin: Coordinate, direction: Coordinate) -> Ray {
        Ray { origin, direction }
    }
}
