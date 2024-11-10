mod vec3;
mod color;
mod ray;
mod sphere;
mod helper;
mod interval;
mod camera;

use camera::Camera;
use sphere::Sphere;
use vec3::Vec3;
use vec3::Point3;
use std::rc::Rc;
use ray::HittableList;

fn main() {
    let mut world : HittableList = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0_f64, 0_f64,-1_f64), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0_f64, -100.5,-1_f64), 100_f64)));
    let mut cam : Camera = Camera::new(16_f64/9_f64, 400);
    cam.render(&world);
}
