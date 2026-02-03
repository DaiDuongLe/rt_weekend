mod vec3;
use vec3::{Vec3, color};

fn main() {
    let mut v = Vec3(2.0, 4.0, 6.0);
    println!("{:?}", Vec3::unit_vector(&v));
    v /= 2.0;
    println!("{:?}", v / 2.0);
    return;
    assert_eq!(-v, Vec3(-1.0, -2.0, -3.0));
    assert_eq!(v, Vec3(1.0, 2.0, 3.0));
    // Image

    let image_width: u16 = 256;
    let image_height: u16 = 256;

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let r: f64 = i as f64 / (image_width - 1) as f64;
            let g: f64 = j as f64 / (image_height - 1) as f64;
            let b: f64 = 0.0;

            let ir: u16 = (255.0 * r) as u16;
            let ig: u16 = (255.0 * g) as u16;
            let ib: u16 = (255.0 * b) as u16;
            println!("{ir} {ig} {ib}");
        }
    }

    eprintln!("\rDone.                 ");
}
