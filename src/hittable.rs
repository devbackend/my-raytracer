use crate::{Ray, Vec3};

pub struct HitRecord {
    point: Vec3,
    normal: Vec3,
    t: f64,
    is_front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64, rec: HitRecord) -> bool;
}

impl HitRecord {
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