use crate::vec::{Color, Coordinate, Vec3};

pub struct Ray {
    origin: Coordinate,
    direction: Coordinate,
}

impl Ray {
    pub fn at(&self, t: f32) -> Coordinate {
        return self.origin + (self.direction * t);
    }

    pub fn intersects_sphere(&self, center: Coordinate, radius: f32) -> f32 {
        let oc = self.origin - center;
        let a = Vec3::dot(&self.direction, &self.direction);
        let b = 2.0 * Vec3::dot(&oc, &self.direction);
        let c = Vec3::dot(&oc, &oc) - (radius * radius);
        let discriminant = (b * b) - (4.0 * a * c);
        if discriminant < 0.0 {
            return -1.0;
        }
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }

    pub fn color(&self) -> Color {
        let t = self.intersects_sphere(Coordinate::new(0.0, 0.0, -1.0), 0.5);
        if t > 0.0 {
            let n = (self.at(t) - Coordinate::new(0.0, 0.0, -1.0)).unit_vec();
            return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
        }
        let unit_dir = self.direction.unit_vec();
        let t = 0.5 * (unit_dir.y + 1.0);
        return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
    }

    pub fn new(origin: Coordinate, direction: Coordinate) -> Ray {
        Ray { origin, direction }
    }
}
