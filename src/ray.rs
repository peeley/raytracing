use crate::hittable::{HitRecord, Hittable, HittableList};
use crate::vec::{Color, Coordinate};
use crate::material::Material;

pub struct Ray {
    pub origin: Coordinate,
    pub direction: Coordinate,
}

impl Ray {
    pub fn at(&self, t: f32) -> Coordinate {
        return self.origin + (self.direction * t);
    }

    pub fn color<U: Material, T: Hittable<U>>(&self, world: &HittableList<T, U>, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::new();
        if world.hit(self, 0.001, std::f32::INFINITY, &mut rec) {
            let bounce_target = rec.point + rec.normal.random_in_hemisphere();
            let bounced_ray = Self::new(rec.point.clone(), bounce_target - rec.point);
            return 0.5 * bounced_ray.color(world, depth-1);
        }
        let unit_dir = self.direction.unit_vec();
        let t = 0.5 * (unit_dir.y + 1.0);
        return (Color::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color::new(0.5, 0.7, 1.0) * t);
    }

    pub fn new(origin: Coordinate, direction: Coordinate) -> Ray {
        Ray { origin, direction }
    }
}
