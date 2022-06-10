use crate::{Ray, Vec3};
use crate::hittable::{HitRecord, Hittable};

pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc = r.get_origin() - self.center;

        let a = r.get_direction().squared_len();
        let half_b = Vec3::dot(oc, r.get_direction());
        let c = oc.squared_len() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return HitRecord::default();
        }

        let sqrt_d = discriminant.sqrt();

        let root = (-half_b - sqrt_d) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrt_d) / a;
            if root < t_min || t_max < root {
                return HitRecord::default();
            }
        }

        let mut rec = HitRecord::default();

        rec.set_t(root);
        rec.set_point(r.point_by(rec.get_t()));
        rec.set_normal((rec.get_point() - self.center) / self.radius);

        let outward_normal = (rec.get_point() - self.center) / self.radius;
        rec.calc_normal_and_fron_face(r, outward_normal);
        rec.set_is_hit(true);

        return rec;
    }
}