use crate::vector;
use crate::ray;
use crate::materials;

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub radius: f64,
    pub center: vector::Vector,
    pub material: materials::Material
}

impl Sphere {
    pub fn hit_ray(self, ray: ray::Ray,
                   t_min: f64, t_max: f64) -> (f64, bool) {
        // We simplify the quadratic eq.:
        // t^2 * (dir*dir)
        //  + t * (dir * (origin of ray - center of sphere))
        //  + ( origin^2 - R^2  )
        let origin_to_center = ray.origin - self.center;
        let a = ray.direction * ray.direction;
        let half_b = origin_to_center * ray.direction;
        let c = origin_to_center * origin_to_center - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;

        if discriminant >= 0. {
            let quadr_solution = (-half_b - discriminant.sqrt()) / a;
            if quadr_solution < t_max && quadr_solution > t_min {
                return (quadr_solution, true)
            } else {
                let quadr_solution = (-half_b + discriminant.sqrt()) / a;
                if quadr_solution < t_max && quadr_solution > t_min {
                    return (quadr_solution, true)
                }
            }
        }
        return (t_min, false)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct HitRecord {
    pub point: vector::Vector,
    pub normal: vector::Vector,
}
