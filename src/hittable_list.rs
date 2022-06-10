use crate::hittable::{HitRecord, Hittable};
use crate::Ray;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new() -> Self {
        HittableList{objects: vec![]}
    }

    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }

    pub fn clear(&mut self) {
        self.objects = vec![]
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: Ray, t_min: f64, t_max: f64) -> HitRecord {
        let mut rec: HitRecord = HitRecord::default();

        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            let obj_rec = obj.hit(r, t_min, closest_so_far);

            if !obj_rec.get_is_hit() {
                continue;
            }

            rec.set_is_hit(true);
            closest_so_far = rec.get_t();
            rec = obj_rec;
        }

        return rec;
    }
}