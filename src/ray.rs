use crate::vec::{Coordinate, Color};

pub struct Ray {
    origin: Coordinate,
    direction: Coordinate
}

impl Ray {
    pub fn at(&self, t: f32) -> Coordinate {
        return self.origin + (self.direction * t);
    }
    pub fn color(&self) -> Color {
        let unit_dir = self.direction.unit_vec();
        let t = 0.5 * (unit_dir.y + 1.0);
        return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
    }
    pub fn new(origin: Coordinate, direction: Coordinate) -> Ray {
        Ray {
            origin,
            direction
        }
    }
}
