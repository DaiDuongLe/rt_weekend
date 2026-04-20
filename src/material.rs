use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: &Vec3) -> Self {
        Self { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction (rand_unit_vec is -rec.normal)
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo;

        true
    }
}

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Vec3, fuzz: f64) -> Self {
        Self {
            albedo: *albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(r_in.direction(), &rec.normal);
        let reflected = Vec3::unit_vector(&reflected) + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;

        Vec3::dot(scattered.direction(), &rec.normal) > 0.0
    }
}
