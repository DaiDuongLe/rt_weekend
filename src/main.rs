mod vec3;
use vec3::*;

mod ray;

mod hittable;
mod hittable_list;
use hittable_list::*;

mod rtweekend;

mod shapes;
use crate::shapes::Sphere;

mod interval;

mod camera;
use camera::*;

mod material;
use material::*;

use std::rc::Rc;

fn main() {
    // World
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(&Vec3(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Vec3(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Metal::new(&Vec3(0.8, 0.8, 0.8), 0.0));
    let material_right = Rc::new(Metal::new(&Vec3(0.8, 0.6, 0.2), 0.0));

    world.add(Box::new(Sphere::new(
        &Vec3(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        &Vec3(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        &Vec3(-1.0, 0.0, -1.3),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        &Vec3(1.0, 0.0, -1.3),
        0.5,
        material_right.clone(),
    )));

    // Camera
    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1920;
    cam.samples_per_pixel = 150;
    cam.max_depth = 50;

    cam.render(&world);
}
