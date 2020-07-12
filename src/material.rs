use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color },
}

pub fn scatter(ray: &Ray, rec: &mut HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
    match rec.material {
        Material::Lambertian { albedo } => {
            let scatter_dir = rec.normal + Vec3::random_unit();
            *scattered = Ray::new(rec.point, scatter_dir);
            *color = albedo;
            return true;
        }
        Material::Metal { albedo } => {
            let reflected = reflect(ray.direction, rec.normal);
            *scattered = Ray::new(rec.point, reflected);
            *color = albedo;
            return Vec3::dot(&scattered.direction, &rec.normal) > 0.0;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * Vec3::dot(&v, &n) * n;
}
