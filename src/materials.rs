use crate::vector;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Diffuse {
        albedo: vector::Vector
    },
    ConstantColor {
        color: vector::Vector
    },
    FancyNormals {},
    Metal {
        albedo: vector::Vector,
        fuzz: f64
    },
    Dielectric {
        refractive_index: f64  // "n" as in
                               // n * sin(ϑ) = n' * sin(ϑ')  
    },
}
