use std::ops::{Add, Div, Mul, Sub};

use rand::Rng;

use crate::{Hittable, Ray};

#[derive(Copy, Clone)]
pub struct Vec3(f64, f64, f64);

const MAX_DEPTH: i32 = 50;

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3(x, y, z)
    }

    pub fn new_by_val(val: f64) -> Self {
        Self::new(val, val, val)
    }

    pub fn unit_vector(from: Self) -> Self {
        from / (from.len() as f64)
    }

    pub fn random_in_unit_sphere() -> Self {
        let mut v = Self::new(rand::thread_rng().gen::<f64>(), rand::thread_rng().gen::<f64>(), rand::thread_rng().gen::<f64>()) * 2.0 - Self::new_by_val(1.0);

        while v.squared_len() >= 1.0 {
            v = Self::new(rand::thread_rng().gen::<f64>(), rand::thread_rng().gen::<f64>(), rand::thread_rng().gen::<f64>()) * 2.0 - Self::new_by_val(1.0)
        }

        return v;
    }

    pub fn reflect(v: Self, n: Self) -> Self {
        v - n * Self::dot(v, n) * 2.0
    }

    pub fn dot(v1: Self, v2: Self) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
    }

    pub fn color(r: Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        let rec = world.hit(r, 0.001, f64::INFINITY);

        if !rec.get_is_hit() {
            let unit_direction = Self::unit_vector(r.get_direction());
            let t = 0.5 * (unit_direction.y() + 1.0);

            return Self::new_by_val(1.0) * (1.0 - t) + Self::new(0.5, 0.7, 1.0) * t;
        }

        if depth >= MAX_DEPTH {
            return Vec3::new_by_val(0.0);
        }

        let m = rec.get_material();

        let scatter = m.scatter(r, &rec);

        if !scatter.get_is_scattered() {
            return Vec3::new_by_val(0.0);
        }

        scatter.get_attenuation() * Self::color(scatter.get_scatter_ray(), world, depth + 1)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn r(&self) -> f64 {
        self.0
    }

    pub fn g(&self) -> f64 {
        self.1
    }

    pub fn b(&self) -> f64 {
        self.2
    }

    pub fn len(&self) -> f64 {
        self.squared_len().sqrt()
    }

    pub fn squared_len(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, v: Self) -> Self::Output {
        Vec3(self.0 + v.0, self.1 + v.1, self.2 + v.2)
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, v: Self) -> Self::Output {
        Vec3(self.0 * v.0, self.1 * v.1, self.2 * v.2)
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, v: Self) -> Self::Output {
        Vec3(self.0 - v.0, self.1 - v.1, self.2 - v.2)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, diff: f64) -> Self::Output {
        Self(self.0 * diff, self.1 * diff, self.2 * diff)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, diff: f64) -> Self::Output {
        self * (1.0 / diff)
    }
}