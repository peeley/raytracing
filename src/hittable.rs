use crate::ray::Ray;
use crate::vec::{Coordinate, Vec3};

pub struct HitRecord {
    pub point: Coordinate,
    pub normal: Vec3,
    pub t: f32
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
