use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::material::{scatter};
use crate::vec::{Color, Coordinate};

pub struct Ray {
    pub origin: Coordinate,
    pub direction: Coordinate,
}

impl Ray {
    pub fn at(&self, t: f32) -> Coordinate {
        return self.origin + (self.direction * t);
    }

    pub fn color<T: Hittable>(&self, world: &HittableList<T>, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::new();
        if world.hit(self, 0.001, std::f32::INFINITY, &mut rec) {
            let mut scattered = Ray::new(
                Coordinate::new(0.0, 0.0, 0.0),
                Coordinate::new(0.0, 0.0, 0.0),
            );
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            if scatter(&self, &mut rec, &mut attenuation, &mut scattered) {
                return attenuation * scattered.color(world, depth - 1);
            }
            return Color::new(0.0, 0.0, 0.0);
        }
        let unit_dir = self.direction.unit_vec();
        let t = 0.5 * (unit_dir.y + 1.0);
        return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
    }

    pub fn new(origin: Coordinate, direction: Coordinate) -> Ray {
        Ray { origin, direction }
    }
}
