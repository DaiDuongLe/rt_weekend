mod vec3;
use vec3::{Vec3, Vec3Enum, color};
mod ray;
use ray::Ray;

fn main() {

    // Image

    let image_width: u16 = 256;
    let image_height: u16 = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            use Vec3 as Color;
            let pixel_color = Color(i as f64 / (image_width - 1) as f64,
                                   j as f64 / (image_height - 1) as f64,
                                   0.0);
            color::write_color(&pixel_color);
        }
    }

    eprintln!("\rDone.                 ");
}
