use crate::{HitRecord, Ray, Vec3};
use crate::material::{Material, ScatteredRecord};

pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz,
        }
    }
}

impl Material for Metal {
    fn as_boxed_copy(&self) -> Box<dyn Material> {
        Box::new(Metal::new(self.albedo, self.fuzz))
    }

    fn scatter(&self, r_in: Ray, hit_r: &HitRecord) -> ScatteredRecord {
        let reflected = Vec3::reflect(Vec3::unit_vector(r_in.get_direction()), hit_r.get_normal());
        let scattered = Ray::new(hit_r.get_point(), reflected + Vec3::random_in_unit_sphere() * self.fuzz);

        ScatteredRecord::new(
            self.albedo,
            scattered,
            Vec3::dot(scattered.get_direction(), hit_r.get_normal()) > 0.0,
        )
    }
}

