use crate::{HitRecord, Ray, Vec3};
use crate::material::{Material, ScatteredRecord};

pub struct Lambertian {
    albedo: Vec3
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Lambertian{
            albedo
        }
    }
}

impl Material for Lambertian {
    fn as_boxed_copy(&self) -> Box<dyn Material> {
        Box::new(Lambertian::new(self.albedo))
    }

    fn scatter(&self, _: Ray, hit_r: &HitRecord) -> ScatteredRecord {
        let target = hit_r.get_point() + hit_r.get_normal() + Vec3::random_in_unit_sphere();

        ScatteredRecord::new(
            self.albedo,
            Ray::new(hit_r.get_point(), target - hit_r.get_point()),
            true
        )
    }
}

