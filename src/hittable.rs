use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Color, Coordinate, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub point: Coordinate,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Material,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: Coordinate::default(),
            normal: Vec3::default(),
            t: 0.0,
            front_face: false,
            material: Material::Lambertian(Color::default()),
        }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, normal: Vec3) {
        self.front_face = Vec3::dot(&ray.direction, &normal) < 0.0;
        self.normal = if self.front_face { normal } else { -normal };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool;
}

pub struct HittableList<T: Hittable> {
    objects: std::vec::Vec<Box<T>>,
}

impl<T: Hittable> HittableList<T> {
    pub fn new(obj: T) -> Self {
        HittableList {
            objects: vec![Box::new(obj)],
        }
    }
    pub fn add(&mut self, obj: T) {
        self.objects.push(Box::new(obj));
    }
}

impl<T: Hittable> Hittable for HittableList<T> {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
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
