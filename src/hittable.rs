use crate::{Ray, Vec3};

#[derive(Clone, Copy)]
pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f64,
    is_hit: bool,
    is_front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRecord;
}

impl HitRecord {
    pub fn default() -> Self {
        HitRecord {
            point: Vec3::new_by_val(0.0),
            normal: Vec3::new_by_val(0.0),
            t: 0.0,
            is_hit: false,
            is_front_face: false,
        }
    }

    pub fn set_is_hit(&mut self, is_hit: bool) {
        self.is_hit = is_hit;
    }

    pub fn get_is_hit(&self) -> bool {
        self.is_hit
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn set_t(&mut self, t: f64) {
        self.t = t;
    }

    pub fn get_point(&self) -> Vec3 {
        self.point
    }

    pub fn set_point(&mut self, point: Vec3) {
        self.point = point;
    }

    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }

    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = normal;
    }

    pub fn calc_normal_and_fron_face(&mut self, r: Ray, normal: Vec3) {
        self.is_front_face = Vec3::dot(r.get_direction(), normal) < 0.0;

        if self.is_front_face {
            self.normal = normal;
        } else {
            self.normal = normal * -1.0;
        }
    }
}