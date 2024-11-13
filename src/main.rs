mod vec3;
mod color;
mod ray;
mod sphere;
mod helper;
mod interval;
mod camera;
mod material;

use camera::Camera;
use material::Dielectric;
use material::{Lambertian, Metal};
use sphere::Sphere;
use vec3::Vec3;
use vec3::Point3;
use std::rc::Rc;
use ray::HittableList;

fn main() {
    let mut world : HittableList = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(Point3::new(0_f64, -100.5,-1_f64), 100.0, material_ground)));
    world.add(Rc::new(Sphere::new(Point3::new(0_f64, 0_f64,-1.2), 0.5, material_center)));
    world.add(Rc::new(Sphere::new(Point3::new(-1.0, 0_f64, -1_f64), 0.5, material_left)));
    world.add(Rc::new(Sphere::new(Point3::new(1.0, 0_f64 ,-1_f64), 0.5, material_right)));
    //aspect ratio, img_width, pixels_per_sample, depth
    let mut cam : Camera = Camera::new(16_f64/9_f64, 400, 100, 50);
    cam.render(&world);
}
