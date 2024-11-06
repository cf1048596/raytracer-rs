use crate::{vec3::Point3, Vec3};


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
