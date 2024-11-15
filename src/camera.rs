use std::{io::{self, Write}, rc::Rc, result};
use crate::{color::{write_color, Color}, helper::{deg_to_rad, random_f64, random_f64_range}, interval::Interval, ray::{HitRecord, Hittable, Ray, Scatter}, vec3::{self, cross, random_in_unit_disk, random_on_hemisphere, random_unit_vector, unit_vector, Point3, Vec3}};
use crate::helper::INFINITY;

pub struct Camera {
    aspect_ratio: f64,
    img_width: i32,
    img_height: i32,
    samples_per_pixel: i32,
    max_depth: i32,
    pixels_sample_scale: f64,
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    vfov: f64, //vertical angle fov
    pub lookfrom: Point3,//point angle looking from
    pub lookat: Point3,  // point angle looking at
    pub vup: Vec3,      // camera relative up direction
    pub defocus_angle: f64,
    pub focus_dist: f64,
    u : Vec3,
    v : Vec3, 
    w : Vec3, 
    defocus_disk_u : Vec3,
    defocus_disk_v : Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, img_width: i32, samples_per_pixel: i32, max_depth: i32, vfov: f64) -> Self {
        Self {
            aspect_ratio,
            img_width,
            img_height: 0,
            samples_per_pixel,
            pixels_sample_scale: 0_f64,
            max_depth,
            center : Point3::new_empty(),
            pixel00_loc : Point3::new_empty(),
            pixel_delta_u : Vec3::new_empty(),
            pixel_delta_v : Vec3::new_empty(),
            lookfrom : Point3::new_empty(),
            lookat : Point3::new(0_f64, 0_f64, -1_f64),
            vup : Point3::new(0_f64, 1_f64, 0_f64),
            u : Point3::new_empty(),
            v : Point3::new_empty(),
            w : Point3::new_empty(),
            vfov,
            defocus_angle : 0_f64,
            focus_dist : 0_f64,
            defocus_disk_u : Vec3::new_empty(),
            defocus_disk_v : Vec3::new_empty(),
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
                let mut pixel_color : Color = Color::new_empty();
                for sample in 0..self.samples_per_pixel {
                    let ray = self.get_ray(x, y);
                    pixel_color += self.ray_color(&ray, world, self.max_depth);
                }
                let resultant_color = self.pixels_sample_scale*pixel_color;
                write_color(&resultant_color);
            }
        }
        eprintln!("\rDone");
    }

    fn init(&mut self) {
        self.img_height = (self.img_width as f64 / self.aspect_ratio) as i32;
        self.img_height = if self.img_height < 1 { 1 } else { self.img_height };

        self.pixels_sample_scale = 1.0/self.samples_per_pixel as f64;
        self.center = self.lookfrom;

        //determine viewport dimensions
        let theta = deg_to_rad(self.vfov);
        let h = (theta/2.0).tan();
        let viewport_height = 2.0*h*self.focus_dist;
        let viewport_width = viewport_height * (self.img_width as f64/self.img_height as f64);

        //calculate our unit vectors u, v, w for the camera coordinate frame
        self.w = unit_vector(&(self.lookfrom - self.lookat));
        self.u = unit_vector(&cross(&self.vup, &self.w)); 
        self.v = cross(&self.w, &self.u);

        //calculate vectors across horizontal and vertical viewpoint edges
        let viewport_u = viewport_width*self.u;
        let viewport_v = viewport_height*(-self.v);

        //calculate dist vectors pixel to pixel horizontally and vertically
        self.pixel_delta_u = viewport_u / self.img_width.into();
        self.pixel_delta_v = viewport_v / self.img_height.into();

        //calculate location of upper left pixel 
        let viewport_upper_left = self.center - (self.focus_dist*self.w) -viewport_u/2_f64 - viewport_v/2_f64;
        self.pixel00_loc = viewport_upper_left + 0.5_f64 * (self.pixel_delta_u + self.pixel_delta_v);

        //calculate camera defocus disk basis vectors
        let defocus_radius : f64 = self.focus_dist * (deg_to_rad(self.defocus_angle/2_f64).tan());
        self.defocus_disk_u = defocus_radius * self.u;
        self.defocus_disk_v = defocus_radius * self.u;
    }

    fn ray_color(&self, ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
        if depth <= 0 {
            return Color::new_empty();
        }

        let mut hit_rec : HitRecord = HitRecord::new_empty();
        if world.hit(ray, Interval::new(0.001, INFINITY), &mut hit_rec) {
            let mut scattered_ray : Ray = Ray::new_empty();
            let mut attenuation : Color =  Color::new_empty();
            if hit_rec.mat.clone().expect("shouldn't crash rite").scatter(ray, &mut hit_rec, &mut attenuation, &mut scattered_ray) {
                return attenuation * self.ray_color(&scattered_ray, world, depth-1);
            }
            return Color::new(0_f64, 0_f64, 0_f64);
        }
        let unit_dir : Vec3 = unit_vector(&ray.dir());
        let a = 0.5_f64*(unit_dir.y() + 1_f64);
        (1_f64-a)*Color::new(1_f64, 1_f64, 1_f64) + a*Color::new(0.5_f64, 0.7_f64, 1_f64)
    }
    
    fn get_ray(&self, x : i32, y: i32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((x as f64 + offset.x()) * self.pixel_delta_u)
            + ((y as f64 + offset.y()) * self.pixel_delta_v);
        let ray_origin = match self.defocus_angle {
            x if x <= 0_f64 => self.center,
            _ => self.defocus_disk_sample(),
        };
        let ray_dir = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_dir)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_f64()-0.5, random_f64()-0.5, 0_f64)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }
}
