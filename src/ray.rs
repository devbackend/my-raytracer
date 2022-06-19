use crate::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Self { origin, direction }
    }

    pub fn default() -> Self {
        Self {
            origin: Vec3::new_by_val(0.0),
            direction: Vec3::new_by_val(0.0),
        }
    }

    pub fn get_origin(&self) -> Vec3 {
        self.origin
    }

    pub fn get_direction(&self) -> Vec3 {
        self.direction
    }

    pub fn point_by(&self, diff: f64) -> Vec3 {
        self.origin + self.direction * diff
    }
}