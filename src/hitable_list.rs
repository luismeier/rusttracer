use crate::Material;
use crate::{
    hitable::{HitRecord, Hitable},
    ray::Ray,
};

pub struct HitableList {
    list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList { list }
    }
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<(HitRecord, &Material)> {
        let mut closest_so_far = t_max;
        let mut hit_anything = None;

        for obj in self.list.iter() {
            if let Some((hit_record, material)) = obj.hit(r, t_min, closest_so_far) {
                closest_so_far = hit_record.t;
                hit_anything = Some((
                    HitRecord::new(hit_record.p, hit_record.normal, hit_record.t),
                    material,
                ));
            }
        }
        hit_anything
    }
}
