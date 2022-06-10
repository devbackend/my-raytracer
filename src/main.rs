use crate::ray::Ray;
use crate::vec3::Vec3;

mod vec3;
mod ray;
mod hittable;
mod sphere;

fn main() {
    println!("{}", draw_pic(600, 300));
}

fn draw_pic(x: i32, y: i32) -> String {
    if x < 1 || y < 1 {
        panic!("bad size")
    }

    let mut res = String::new();
    res.push_str(format!("P3\n{} {}\n255\n", x, y).as_str());

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..y).rev() {
        for i in 0..x {
            let u = i as f64 / x as f64;
            let v = j as f64 / y as f64;

            let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col: Vec3 = color(r);

            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;

            res.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }

    return res;
}

fn color(r: Ray) -> Vec3 {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r.clone());

    if t > 0.0 {
        let new_v = Vec3::unit_vector(r.point_by(t) - Vec3::new(0.0, 0.0, -1.0));
        return Vec3::new(new_v.x() + 1.0, new_v.y() + 1.0, new_v.z() + 1.0) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(r.get_direction());
    let t: f64 = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new_by_val(1.0) * (1.0 - t) + Vec3::new(0.2, 0.7, 1.0) * t
}

fn hit_sphere(center: Vec3, radius: f64, r: Ray) -> f64 {
    let v = r.get_origin() - center;

    let a = r.get_direction().squared_len();
    let half_b = Vec3::dot(v, r.get_direction());
    let c = v.squared_len() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    }

    (half_b - discriminant.sqrt()) / a
}