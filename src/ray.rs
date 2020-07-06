use crate::vec::{Color, Coordinate};

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
        let a = self.direction.dot(&self.direction);
        let b = 2.0 * oc.dot(&self.direction);
        let c = oc.dot(&oc) - (radius * radius);
        let discriminant = (b * b) - (4.0 * a * c);
        eprintln!("{}", discriminant);
        return discriminant > 0.0;
    }
    pub fn new(origin: Coordinate, direction: Coordinate) -> Ray {
        Ray { origin, direction }
    }
}
