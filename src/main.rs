use rand::Rng;

use crate::camera::Camera;
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
mod camera;

fn main() {
    println!("{}", draw_pic(600, 300));
}

fn draw_pic(x: i32, y: i32) -> String {
    let ns = 100;

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

    let cam: Camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(-2.0, -1.0, -1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
    );

    for j in (0..y).rev() {
        for i in 0..x {
            let mut col = Vec3::new_by_val(0.0);
            for _ in 0..ns {
                let u = (i as f64 + rand::thread_rng().gen::<f64>()) / x as f64;
                let v = (j as f64 + rand::thread_rng().gen::<f64>()) / y as f64;

                let r: Ray = cam.get_ray(u, v);

                col = col + color(r, &world);
            }

            col = col / ns as f64;
            col = Vec3::new(col.r().sqrt(), col.g().sqrt(), col.b().sqrt());

            let ir: i32 = (255.99 * col.r()) as i32;
            let ig: i32 = (255.99 * col.g()) as i32;
            let ib: i32 = (255.99 * col.b()) as i32;

            res.push_str(format!("{} {} {}\n", ir, ig, ib).as_str());
        }
    }

    return res;
}

fn color(r: Ray, world: &dyn Hittable) -> Vec3 {
    let rec = world.hit(r, 0.001, f64::INFINITY);

    if rec.get_is_hit() {
        let target = rec.get_point() + rec.get_normal() + Vec3::random_in_unit_sphere();
        return color(Ray::new(rec.get_point(), target - rec.get_point()), world) * 0.5;
    }

    let unit_direction = Vec3::unit_vector(r.get_direction());

    let t = 0.5 * (unit_direction.y() + 1.0);

    Vec3::new_by_val(1.0) * (1.0 - t) + Vec3::new(0.5, 0.7, 1.0) * t
}
