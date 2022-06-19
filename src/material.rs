use crate::{HitRecord, Ray, Vec3};

pub trait Material {
    fn as_boxed_copy(&self) -> Box<dyn Material>;
    fn scatter(&self, r_in: Ray, hit_r: &HitRecord) -> ScatteredRecord;
}

pub struct DefaultMaterial {}

pub struct ScatteredRecord {
    attenuation: Vec3,
    scatter_ray: Ray,
    is_scattered: bool,
}

impl ScatteredRecord {
    pub fn new(attenuation: Vec3, scatter_ray: Ray, is_scattered: bool) -> Self {
        ScatteredRecord { attenuation, scatter_ray, is_scattered }
    }

    pub fn get_attenuation(&self) -> Vec3 {
        self.attenuation
    }
    pub fn get_scatter_ray(&self) -> Ray {
        self.scatter_ray
    }
    pub fn get_is_scattered(&self) -> bool {
        self.is_scattered
    }
}

impl Material for DefaultMaterial {
    fn as_boxed_copy(&self) -> Box<dyn Material> {
        Box::new(DefaultMaterial::new())
    }

    fn scatter(&self, r_in: Ray, _: &HitRecord) -> ScatteredRecord {
        ScatteredRecord::new(Vec3::new_by_val(0.0), r_in, false)
    }
}

impl DefaultMaterial {
    pub fn new() -> Self {
        DefaultMaterial{}
    }
}