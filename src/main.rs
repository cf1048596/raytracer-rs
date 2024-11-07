mod vec3;
mod color;
mod ray;


use color::write_color;
use color::Color;
use vec3::dot;
use vec3::unit_vector;
use vec3::Vec3;
use vec3::Point3;
use std::io::{self, Write};
use std::mem::Discriminant;
use ray::Ray;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = *center - r.origin();
    let a = dot(&r.dir(), &r.dir());
    let b = -2_f64 * dot(&r.dir(), &oc);
    let c = dot(&oc, &oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4_f64*a*c;
    discriminant >= 0_f64
}


fn ray_color(ray: &Ray) -> Color {
    let center_point : Point3 = Point3::new(0_f64, 0_f64, -1_f64);
    if hit_sphere(&center_point, 0.5_f64, ray) {
        Color::new(1_f64, 0_f64, 0_f64)
    } else {
        let unit_direction : Vec3 = unit_vector(&ray.dir());
        let a = 0.5*unit_direction.y()+1.0;
        (1.0-a)*Color::new(1.0, 1.0, 1.0) + a*Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {

    //image
    let aspect_ratio = 16.0 / 9.0;
    let img_width = 400;

    //calculate image height and make sure at the very minimum it's 1
    let img_height = (img_width/aspect_ratio as u32).max(1);


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
            let mut r : Ray = Ray::new(camera_center, ray_direction);
            let pixel_color : Color = ray_color(&r); 
            write_color(&pixel_color);
        }
    }
    eprintln!("\rDone");
}
