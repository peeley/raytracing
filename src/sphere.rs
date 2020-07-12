use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Coordinate, Vec3};

pub struct Sphere {
    center: Coordinate,
    radius: f32,
    material: Material,
}

impl Sphere {
    pub fn new(center: Coordinate, radius: f32, material: Material) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.at(temp);
                let outward_normal = (rec.point - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.material = self.material;
                return true;
            }
            temp = (-half_b + root) / a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.at(temp);
                let outward_normal = (rec.point - self.center) / self.radius;
                rec.set_face_normal(ray, outward_normal);
                rec.material = self.material;
                return true;
            }
        }
        return false;
    }
}
