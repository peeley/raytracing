use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::Color;

pub trait Material {
    fn scatter<T: Material>(ray: &Ray, rec: &HitRecord<T>, color: &Color, scattered: &Ray) -> bool;
}
