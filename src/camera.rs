use std::rc::Rc;

use crate::hittable::*;
// use crate::hittable_list::*;
use crate::interval::*;
use crate::material::*;
use crate::ray::*;
use crate::rtweekend::*;
use crate::vec3::*;

pub struct Camera {
    image_height: u32,        // Rendered image height
    pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples
    center: Vec3,             // Camera center
    pixel00_loc: Vec3,        // Location of pixel 0, 0
    pixel_delta_u: Vec3,      // Offset to pixel to the right
    pixel_delta_v: Vec3,      // Offset to pixel below
    pub aspect_ratio: f64,    // Ratio of image width over height
    pub image_width: u32,     // Rendered image width in pixel count
    pub samples_per_pixel: u32,
    pub max_depth: u32,
}

impl Camera {
    pub fn new() -> Self {
        Self {
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Vec3(0.0, 0.0, 0.0),
            pixel00_loc: Vec3(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3(0.0, 0.0, 0.0),
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
        }
    }

    pub fn render(&mut self, world: &impl Hittable) {
        self.initialize();

        println!("P3\n{} {}\n255", self.image_width, self.image_height);

        for j in 0..self.image_height {
            eprint!("\rScanlines remaining: {} ", self.image_height - j);
            for i in 0..self.image_width {
                let mut pixel_color = Vec3(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                // Averaging the sample
                color::write_color(&(self.pixel_samples_scale * pixel_color));
            }
        }

        eprintln!("\rDone.                 ");
    }

    fn initialize(&mut self) {
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = Vec3(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate vectors across viewport edges
        let viewport_u = Vec3(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3(0.0, -viewport_height, 0.0);

        // Calculate delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / self.image_width as f64;
        let pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate location of upper left viewport point and pixel
        let viewport_upper_left =
            self.center - Vec3(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at a randomly
        // sampled point around the pixel location i, j

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.0) * self.pixel_delta_u)
            + ((j as f64 + offset.1) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square() -> Vec3 {
        // <-0.5, 0.5> square sample region around the pixel (unit square)
        Vec3(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, depth: u32, world: &impl Hittable) -> Vec3 {
        // No more light is gathered when ray bounce limit is exceeded
        if depth <= 0 {
            return Vec3(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord {
            p: Vec3(0.0, 0.0, 0.0),
            normal: Vec3(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            mat: Rc::new(Lambertian::new(&Vec3(0.0, 0.0, 0.0))),
        };

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(&Vec3(0.0, 0.0, 0.0), &Vec3(0.0, 0.0, 0.0));
            let mut attenuation = Vec3(0.0, 0.0, 0.0);
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            }
            return Vec3(0.0, 0.0, 0.0);
        }

        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        // linear blend/interpolation (lerp) between white and light blue
        (1.0 - a) * Vec3(1.0, 1.0, 1.0) + a * Vec3(0.5, 0.7, 1.0)
    }
}
