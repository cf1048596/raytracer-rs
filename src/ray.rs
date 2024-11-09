use std::{fmt::Pointer, rc::Rc, sync::Arc};

use crate::{vec3::{dot, Point3}, Vec3};

#[derive(Copy, Clone, Debug)]
pub struct HitRecord {
    pub p : Point3,
    pub normal: Vec3,
    pub t : f64,
    pub front_face : bool,
}

pub struct Ray {
    origin: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(orig: Point3, direction: Vec3) -> Ray {
        Ray {
            origin: orig,
            dir : direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.origin
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t*self.dir
    }
}

pub trait Hittable {
    fn hit(&self, ray :&Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
}

pub trait SetFaceNormal {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3);
}

impl SetFaceNormal for HitRecord {
    fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&ray.dir(), outward_normal) < 0_f64;
        match self.front_face {
            true => self.normal = *outward_normal,
            false => self.normal = -*outward_normal
        }
    }
}

impl HitRecord {
    pub fn new_empty() -> HitRecord {
        HitRecord {
            p : Point3::new(0_f64, 0_f64, 0_f64),
            normal : Vec3::new(0_f64, 0_f64, 0_f64),
            t : 0_f64,
            front_face : false,
        }
    }
}


pub struct HittableList {
    objects : Vec<Rc<dyn Hittable>>,
}

impl HittableList {

}

impl Hittable for HittableList {
    fn hit(&self, ray :&Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut temp_rec : HitRecord = HitRecord::new_empty();
        let mut hit_anything : bool = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(ray, ray_tmin, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec
            }
        }
        hit_anything
    }
}
