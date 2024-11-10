mod vec3;
mod color;
mod ray;
mod sphere;
mod helper;
mod interval;

use color::write_color;
use helper::INFINITY;
use ray::HitRecord;
use ray::Hittable;
use sphere::Sphere;
use color::Color;
use vec3::dot;
use vec3::unit_vector;
use vec3::Vec3;
use vec3::Point3;
use std::io::{self, Write};
use std::rc::Rc;
use ray::Ray;
use ray::HittableList;

fn ray_color(ray: &Ray,  world: &dyn Hittable) -> Color {
    let mut hit_rec : HitRecord = HitRecord::new_empty();
    if world.hit(ray, 0_f64, INFINITY, &mut hit_rec) {
        return 0.5_f64 * (hit_rec.normal + Color::new(1_f64, 1_f64, 1_f64));
    }
    let unit_dir : Vec3 = unit_vector(&ray.dir());
    let a = 0.5_f64*(unit_dir.y() + 1_f64);
    (1_f64-a)*Color::new(1_f64, 1_f64, 1_f64) + a*Color::new(0.5_f64, 0.7_f64, 1_f64)
}

fn main() {

    //image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;
    let mut world : HittableList = HittableList::new();

    //calculate image height and make sure at the very minimum it's 1
    let mut img_height = (img_width as f64 / aspect_ratio) as i32;
    img_height = if img_height < 1 { 1 } else { img_height };
    world.add(Rc::new(Sphere::new(Point3::new(0_f64, 0_f64,-1_f64), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0_f64, -100.5,-1_f64), 100_f64)));

    //camera details
    let focal_len = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (img_width as f64/img_height as f64);
    let camera_center = Point3::new(0_f64, 0_f64, 0_f64);

    //calculate vectors across horizontal and vertical viewpoint edges
    let viewport_u = Vec3::new(viewport_width, 0_f64, 0_f64);
    let viewport_v = Vec3::new(0_f64, -viewport_height, 0_f64);

    //calculate dist vectors pixel to pixel horizontally and vertically
    let pixel_delta_u = viewport_u / img_width.into();
    let pixel_delta_v = viewport_v / img_height.into();

    //calculate location of upper left pixel 
    let viewport_upper_left = camera_center - Vec3::new(0_f64, 0_f64, focal_len) -viewport_u/2_f64 - viewport_v/2_f64;
    let pixel00_loc = viewport_upper_left + 0.5_f64 * (pixel_delta_u + pixel_delta_v);

    
    //write header to stdout
    println!("P3");
    println!("{} {}", img_width, img_height);
    println!("255");
    for y in 0..img_height {

        eprint!("\rScanlines remaining: {}", img_height - y);
        io::stderr().flush().unwrap(); // Ensure the progress is displayed immediately

        for x in 0..img_width {
            let pixel_center = pixel00_loc + (x as f64 *pixel_delta_u) + (y as f64 *pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r : Ray = Ray::new(camera_center, ray_direction);
            let pixel_color : Color = ray_color(&r, &world);
            write_color(&pixel_color);
        }
    }
    eprintln!("\rDone");
}
