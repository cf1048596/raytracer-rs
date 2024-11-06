mod vec3;
mod color;
mod ray;


use color::write_color;
use color::Color;
use vec3::Vec3;
use std::io::{self, Write};
use ray::Ray;


fn ray_color(ray: &mut Ray) -> Color {
    Color::new(0_f64,0_f64,0_f64)

}

fn main() {

    //width & height
    let img_width : u16 = 256;
    let img_height : u16 = 256;
    // Write header to stdout
    println!("P3");
    println!("{} {}", img_width, img_height);
    println!("255");
    for y in 0..img_height {

        eprint!("\rScanlines remaining: {}", img_height - y);
        io::stderr().flush().unwrap(); // Ensure the progress is displayed immediately

        for x in 0..img_width {
            let pixel_color : Vec3 = Vec3::new(x as f64/(img_width-1) as f64, y as f64/(img_height-1) as f64, 0_f64);
            write_color(&pixel_color);
        }
    }
    eprintln!("\rDone");
}
