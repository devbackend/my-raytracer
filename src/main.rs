use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::ray::Ray;
use crate::sphere::Sphere;
use crate::vec3::Vec3;

mod vec3;
mod ray;
mod hittable;
mod sphere;
mod hittable_list;

fn main() {
    println!("{}", draw_pic(600, 300));
}

fn draw_pic(x: i32, y: i32) -> String {
    if x < 1 || y < 1 {
        panic!("bad size")
    }

    let mut res = String::new();
    res.push_str(format!("P3\n{} {}\n255\n", x, y).as_str());

    let sph = Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0);

    let mut world = HittableList::new();
    world.add(Box::new(sph));
    world.add(Box::new(ground));

    let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.0, 0.0);
    let origin = Vec3::new(0.0, 0.0, 0.0);

    for j in (0..y).rev() {
        for i in 0..x {
            let u = i as f64 / x as f64;
            let v = j as f64 / y as f64;

            let r: Ray = Ray::new(origin, lower_left_corner + horizontal * u + vertical * v);
            let col: Vec3 = color(r, &world);

            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;

            res.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }

    return res;
}

fn color(r: Ray, world: &dyn Hittable) -> Vec3 {
    let rec = world.hit(r, 0.0, f64::INFINITY);

    if rec.get_is_hit() {
        return (rec.get_normal() + Vec3::new_by_val(1.0)) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(r.get_direction());

    let t = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new_by_val(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
