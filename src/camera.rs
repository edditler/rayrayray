use crate::vector;
use crate::ray;

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub lower_left: vector::Vector,
    pub horizontal: vector::Vector,
    pub vertical: vector::Vector,
    pub origin: vector::Vector,
}

impl Camera {
    // This routine takes a ray and tells us if we hit it.
    // It's just the solution to a quadratic equation
    // The result is a HitRecord, which tells us where we hit, what the normal is and what color results.
    // t_min and t_max tell us where our ray is allowed to be.
    pub fn get_ray(self, u: f64, v: f64) -> ray::Ray {
        return ray::Ray{
                origin: self.origin,
                direction: self.lower_left + u * self.horizontal + v * self.vertical - self.origin
            };
    }
}
