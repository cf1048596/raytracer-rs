use crate::{interval::Interval, Vec3};

pub type Color = Vec3;


pub fn write_color(pixel_color: &Color) {
    let r : f64 = pixel_color.x();
    let g : f64 = pixel_color.y();
    let b : f64 = pixel_color.z();

    let intensity : Interval = Interval::new(0_f64, 0.999);
    let ir : u32 = (256_f64* intensity.clamp(r)) as u32;
    let ig : u32 = (256_f64* intensity.clamp(g)) as u32;
    let ib : u32 = (256_f64* intensity.clamp(b)) as u32;
    println!("{} {} {}", ir, ig, ib);
}
