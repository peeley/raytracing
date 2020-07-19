use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};
use rand::{thread_rng, Rng};

#[derive(Clone, Copy)]
pub enum Material {
    Lambertian(Color),
    Metal(Color, f32),
    Dielectric(f32),
}

pub fn scatter(ray: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
    match rec.material {
        Material::Lambertian(albedo) => {
            let scatter_dir = rec.normal + Vec3::random_unit();
            *scattered = Ray::new(rec.point, scatter_dir);
            *color = albedo;
            return true;
        }
        Material::Metal(albedo, fuzziness) => {
            let reflected = reflect(ray.direction, rec.normal);
            *scattered = Ray::new(
                rec.point,
                reflected + fuzziness * Vec3::random_in_unit_sphere(),
            );
            *color = albedo;
            return Vec3::dot(&scattered.direction, &rec.normal) > 0.0;
        }
        Material::Dielectric(ref_idx) => {
            *color = Color::new(1.0, 1.0, 1.0);
            let eta_quotient = if rec.front_face {
                1.0 / ref_idx
            } else {
                ref_idx
            };
            let unit_dir = ray.direction.unit_vec();
            let cos_theta = Vec3::dot(&(-unit_dir), &rec.normal).min(1.0);
            let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
            if (eta_quotient * sin_theta) > 1.0 {
                let reflected = reflect(unit_dir, rec.normal);
                *scattered = Ray::new(rec.point, reflected);
                return true;
            }
            let reflect_prob = schlick(cos_theta, eta_quotient);
            let mut rng = thread_rng();
            if rng.gen_range(0.0, 1.0) < reflect_prob {
                let reflected = reflect(unit_dir, rec.normal);
                *scattered = Ray::new(rec.point, reflected);
                return true;
            }
            let refracted = refract(unit_dir, rec.normal, eta_quotient);
            *scattered = Ray::new(rec.point, refracted);
            return true;
        }
    }
}

pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    return v - 2.0 * Vec3::dot(&v, &n) * n;
}

pub fn refract(uv: Vec3, n: Vec3, eta_quotient: f32) -> Vec3 {
    let cos_theta = Vec3::dot(&(-uv), &n);
    let r_out_parallel = eta_quotient * (uv + cos_theta * n);
    let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * n;
    return r_out_parallel + r_out_perp;
}

pub fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    return r0 + (1.0 - r0) * (1.0 - cosine).powi(5);
}
