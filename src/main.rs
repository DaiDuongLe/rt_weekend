mod vec3;
use vec3::{Vec3, Vec3Enum, color};
mod ray;
use ray::Ray;
mod hittable;
mod hittable_list;
mod rtweekend;
mod shapes;

fn hit_sphere(center: &Vec3, radius: f64, r: &Ray) -> Option<f64> {
    let oc: Vec3 = *center - *r.origin();
    let a = r.direction().length_squared();
    let h = Vec3::dot(r.direction(), &oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = h * h - a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((h - discriminant.sqrt()) / a)
    }
}

fn ray_color(r: &Ray) -> Vec3 {
    let center = Vec3(0.0, 0.0, -1.0);
    let radius = 0.5;
    if let Option::Some(t) = hit_sphere(&center, radius, r) {
        let N = Vec3::unit_vector(&(r.at(t) - center));
        return Vec3(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) / 2.0;
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    // linear blend/interpolation (lerp) between white and light blue
    (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * Vec3(0.5, 0.7, 1.0)
}

fn main() {
    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: u16 = 400;
    let image_height: u16 = (image_width as f64 / aspect_ratio) as u16;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Vec3(0.0, 0.0, 0.0);

    // Calculate vectors across viewport edges
    let viewport_u = Vec3(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3(0.0, -viewport_height, 0.0);

    // Calculate delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate location of upper left viewport point and pixel
    let viewport_upper_left =
        camera_center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    println!("P3\n{image_width} {image_height}\n255");

    for j in 0..image_height {
        eprint!("\rScanlines remaining: {} ", image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r);
            color::write_color(&pixel_color);
        }
    }

    eprintln!("\rDone.                 ");
}
