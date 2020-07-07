use crate::ray::Ray;
use crate::vec::{Coordinate, Vec3};

pub struct HitRecord {
    pub point: Coordinate,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, normal: Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}
