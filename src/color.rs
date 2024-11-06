use crate::Vec3;

pub type Color = Vec3;


pub fn write_color(pixel_color: &Color) -> () {
    let r : f64 = pixel_color.x();
    let g : f64 = pixel_color.y();
    let b : f64 = pixel_color.z();
    let ir : u32 = (255.999 * r) as u32;
    let ig : u32 = (255.999 * g) as u32;
    let ib : u32 = (255.999 * b) as u32;
    println!("{} {} {}", ir, ig, ib);
    
}
