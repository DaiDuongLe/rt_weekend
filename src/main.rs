mod vec3;
use vec3::{Vec3, color};

mod ray;
use ray::Ray;

mod hittable;
use hittable::*;
mod hittable_list;
use hittable_list::*;

mod rtweekend;
use rtweekend::*;

mod shapes;
use crate::shapes::Sphere;

mod interval;
use interval::*;

mod camera;
use camera::*;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(&Vec3(0.0, 0.0, -1.0), 0.5 as f64)));
    world.add(Box::new(Sphere::new(&Vec3(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1920;

    cam.render(&world);
}
