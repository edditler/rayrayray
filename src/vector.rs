use std::ops::{Add,AddAssign,Sub,Mul,Div,Neg};

#[derive(Clone, Copy, Debug)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vector {
    pub fn norm2(self) -> f64 {
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }

    pub fn unit_vector(self) -> Vector {
        let norm = self.norm2();
        Vector{ x: self.x / norm, y: self.y / norm, z: self.z / norm }
    }
}


impl Sub for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {x: self.x - other.x,
                y: self.y - other.y,
                z: self.z - other.z}
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Vector {
        Vector {
            x: - self.x,
            y: - self.y,
            z: - self.z,
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {x: self.x + other.x,
                y: self.y + other.y,
                z: self.z + other.z}
    }
}

impl Add<f64> for Vector {
    type Output = Vector;

    fn add(self, other: f64) -> Vector {
        Vector {x: self.x + other,
                y: self.y + other,
                z: self.z + other}
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, other: f64) -> Vector {
        Vector {x: self.x * other,
                y: self.y * other,
                z: self.z * other}
    }
}


impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {x: self * other.x,
                y: self * other.y,
                z: self * other.z}
    }
}

impl Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, other: Vector) -> f64 {
        self.x*other.x + self.y*other.y + self.z*other.z
    }
}

impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, other: f64) -> Vector {
        Vector {x: self.x / other,
                y: self.y / other,
                z: self.z / other}
    }
}

