use crate::vector;
use crate::ray;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub radius: f64,
    pub center: vector::Vector,
    pub color: vector::Vector
}

impl Sphere {
    pub fn hit_ray(self, ray: ray::Ray,
                   t_min: f64, t_max: f64) -> HitRecord {
        let mut record = HitRecord{
            t: 10000.,
            point: vector::Vector{x: 0., y: 0., z: 0.},
            normal: vector::Vector{x: 1., y: 0., z: 0.},
            color: vector::Vector{x: 1., y: 1., z: 1.},
            hit_anything: false
        };

        let origin_to_center = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let b = origin_to_center * ray.direction;
        let c = origin_to_center * origin_to_center - self.radius*self.radius;
        let discriminant = b*b - a*c;

        if discriminant > 0. {
            let quadr_solution = (-b - discriminant.sqrt()) / a;
            if quadr_solution < t_max && quadr_solution > t_min {
                record.t = quadr_solution;
                record.point = ray.point_at_parameter(record.t);
                record.normal = (record.point - self.center) / self.radius;
                record.hit_anything = true;
                record.color = self.color;
                println!("a {:?}", self.color);
            } else {
                let quadr_solution = (-b + discriminant.sqrt()) / a;
                if quadr_solution < t_max && quadr_solution > t_min {
                    record.t = quadr_solution;
                    record.point = ray.point_at_parameter(record.t);
                    record.normal = (record.point - self.center) / self.radius;
                    record.hit_anything = true;
                    record.color = self.color;
                    println!("b {:?}", self.color);
                }
            }
        }
        return record
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub t: f64,
    pub point: vector::Vector,
    pub normal: vector::Vector,
    pub color: vector::Vector,
    pub hit_anything: bool
}
