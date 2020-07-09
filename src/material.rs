use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Color;

pub trait Material {
    fn scatter(ray: &Ray, rec: &HitRecord, color: &Color, scattered: &Ray) -> bool;
}
