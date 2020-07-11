use crate::ray::Ray;
use crate::vec::{Coordinate, Vec3};
use crate::material::Material;

#[derive(Clone)]
pub struct HitRecord<T: Material> {
    pub point: Coordinate,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Box<T>,
}

impl<T: Material> HitRecord<T> {
    pub fn new() -> Self {
        HitRecord {
            point: Coordinate::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: Box::new()
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, normal: Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal };
    }
    pub fn set_material(&mut self, mat: T) {
        self.material = Box::new(mat);
    }
}

pub trait Hittable<T: Material> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<T>) -> bool;
}

pub struct HittableList<U: Material, T: Hittable<U>> {
    objects: std::vec::Vec<Box<T>>,
}

impl<U: Material, T: Hittable<U>> HittableList<T, U> {
    pub fn new(obj: T) -> Self {
        HittableList {
            objects: vec![Box::new(obj)],
        }
    }
    pub fn add(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}

impl<U: Material, T: Hittable<U>> Hittable<U> for HittableList<T, U> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord<T>) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut has_hit = false;
        let mut closest = t_max;
        for obj in &self.objects {
            if (*obj).hit(ray, t_min, closest, &mut temp_rec) {
                has_hit = true;
                closest = temp_rec.t;
                *rec = temp_rec.clone();
            }
        }
        return has_hit;
    }
}
