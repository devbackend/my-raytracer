use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone)]
pub struct Vec3(f64, f64, f64);

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

    pub fn dot(v1: Self, v2: Self) -> f64 {
        v1.0 * v2.0 + v1.1 * v2.1 + v1.2 * v2.2
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