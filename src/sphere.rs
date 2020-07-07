use crate::vec::{Coordinate, Vec3};
use crate::ray::Ray;
use crate::hittable::{Hittable, HitRecord};

pub struct Sphere {
    center: Coordinate,
    radius: f32
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32, rec: &mut HitRecord) -> bool {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = Vec3::dot(&oc, &ray.direction);
        let c = oc.length_squared() - (self.radius*self.radius);
        let discriminant = half_b*half_b - a*c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            let mut temp = (-half_b - root)/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.at(temp);
                rec.normal = (rec.point - self.center) / self.radius;
            }
            temp = (-half_b + root)/a;
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.point = ray.at(temp);
                rec.normal = (rec.point - self.center) / self.radius;
            }
            return true;
        }
        return false;
    }
}
