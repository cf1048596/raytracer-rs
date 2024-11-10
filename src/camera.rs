use std::io::{self, Write};
use crate::{color::{write_color, Color}, interval::Interval, ray::{HitRecord, Hittable, Ray}, vec3::{unit_vector, Point3, Vec3}};
use crate::helper::INFINITY;

pub struct Camera {
    aspect_ratio : f64,
    img_width : i32,
    img_height : i32,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, img_width: i32) -> Self {
        Self {
            aspect_ratio,
            img_width,
            img_height: 0,
            center : Point3::new_empty(),
            pixel00_loc : Point3::new_empty(),
            pixel_delta_u : Vec3::new_empty(),
            pixel_delta_v : Vec3::new_empty(),
        }
    }

    pub fn render(&mut self, world : &dyn Hittable) {
        self.init();
        
        //write header to stdout
        println!("P3");
        println!("{} {}", self.img_width, self.img_height);
        println!("255");
        for y in 0..self.img_height {

            eprint!("\rScanlines remaining: {}", self.img_height - y);
            io::stderr().flush().unwrap(); // Ensure the progress is displayed immediately

            for x in 0..self.img_width {
                let pixel_center = self.pixel00_loc + (x as f64 *self.pixel_delta_u) + (y as f64 *self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r : Ray = Ray::new(self.center, ray_direction);
                let pixel_color : Color = self.ray_color(&r, world);
                write_color(&pixel_color);
            }
        }
        eprintln!("\rDone");
    }

    fn init(&mut self) {
        self.img_height = (self.img_width as f64 / self.aspect_ratio) as i32;
        self.img_height = if self.img_height < 1 { 1 } else { self.img_height };
        self.center = Point3::new(0_f64, 0_f64, 0_f64);

        let focal_len = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.img_width as f64/self.img_height as f64);

        //calculate vectors across horizontal and vertical viewpoint edges
        let viewport_u = Vec3::new(viewport_width, 0_f64, 0_f64);
        let viewport_v = Vec3::new(0_f64, -viewport_height, 0_f64);

        //calculate dist vectors pixel to pixel horizontally and vertically
        self.pixel_delta_u = viewport_u / self.img_width.into();
        self.pixel_delta_v = viewport_v / self.img_height.into();

        //calculate location of upper left pixel 
        let viewport_upper_left = self.center - Vec3::new(0_f64, 0_f64, focal_len) -viewport_u/2_f64 - viewport_v/2_f64;
        self.pixel00_loc = viewport_upper_left + 0.5_f64 * (self.pixel_delta_u + self.pixel_delta_v);
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable) -> Color {
        let mut hit_rec : HitRecord = HitRecord::new_empty();
        if world.hit(ray, Interval::new(0_f64, INFINITY), &mut hit_rec) {
            return 0.5_f64 * (hit_rec.normal + Color::new(1_f64, 1_f64, 1_f64));
        }
        let unit_dir : Vec3 = unit_vector(&ray.dir());
        let a = 0.5_f64*(unit_dir.y() + 1_f64);
        (1_f64-a)*Color::new(1_f64, 1_f64, 1_f64) + a*Color::new(0.5_f64, 0.7_f64, 1_f64)
    }
}
