use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::vec::{Color, Coordinate, Vec3};

pub struct Ray {
    pub origin: Coordinate,
    pub direction: Coordinate,
}

impl Ray {
    pub fn at(&self, t: f32) -> Coordinate {
        return self.origin + (self.direction * t);
    }

    pub fn color<T: Hittable>(&self, world: &HittableList<T>) -> Color {
        let mut rec = HitRecord::new();
        if world.hit(self, 0.0, 1000.0, &mut rec) {
            return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
        }
        let unit_dir = self.direction.unit_vec();
        let t = 0.5 * (unit_dir.y + 1.0);
        return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
    }

    pub fn new(origin: Coordinate, direction: Coordinate) -> Ray {
        Ray { origin, direction }
    }
}
