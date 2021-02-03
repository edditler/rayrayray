use crate::vector;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: vector::Vector,
    pub direction: vector::Vector,
}

impl Ray {
    pub fn point_at_parameter(self, t: f64) -> vector::Vector {
        self.origin + t * self.direction
    }
}
