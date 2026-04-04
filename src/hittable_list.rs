use std::rc::Rc;

use crate::Vec3;
use crate::hittable::*;

struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    fn clear(&mut self) {
        self.objects.clear();
    }

    fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &crate::ray::Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;

        for object in &self.objects {
            if object.hit(r, ray_tmin, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }

        hit_anything
    }
}
