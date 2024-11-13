use crate::{color::Color, ray::{HitRecord, Ray, Scatter}, vec3::{dot, random_unit_vector, reflect, unit_vector}};

pub struct Lambertian {
    albedo: Color
}

impl Lambertian {
    pub fn new(color : Color) -> Self  {
        Self {
        albedo: color,
        }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in : &Ray, hit_rec: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool {
        let mut scatter_direction = hit_rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_rec.normal;
        }
        *scattered_ray = Ray::new(hit_rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(color : Color, fuzz_factor: f64) -> Self  {
        let result = if fuzz_factor < 1.0 { fuzz_factor } else { 1.0 };
        Self {
        albedo: color,
        fuzz : result,
        }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in : &Ray, hit_rec: &HitRecord, attenuation: &mut Color, scattered_ray: &mut Ray) -> bool {
        let mut reflected = reflect(&ray_in.dir(), &hit_rec.normal);
        reflected = unit_vector(&reflected) + (self.fuzz * random_unit_vector());
        *scattered_ray = Ray::new(hit_rec.p, reflected);
        *attenuation = self.albedo;
        dot(&scattered_ray.dir(), &hit_rec.normal) > 0_f64
    }
}
