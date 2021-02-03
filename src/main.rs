use std::fs::File;
use std::io::prelude::*;

mod vector;
mod ray;
mod canvas;

fn color(ray: ray::Ray, world: &Vec<canvas::Sphere>) -> vector::Vector {
    let mut record = canvas::HitRecord{
        t: 10000.,
        point: vector::Vector{x: 0., y: 0., z: 0.},
        normal: vector::Vector{x: 1., y: 0., z: 0.},
        color: vector::Vector{x: 1., y: 1., z: 1.},
        hit_anything: false
    };

    let mut closest_so_far = record.t;
    let mut hit_anything = false;

    for sphere in world.iter() {
        record = sphere.hit_ray(ray, 0., closest_so_far);
        if record.hit_anything  {
            closest_so_far = record.t;
            hit_anything = true;
        }
    }

    if hit_anything {
        println!("did hit anything {:?}", record.color);
        let color = vector::Vector{
            x: 0.5 * (record.normal.x + 1.) * record.color.x,
            y: 0.5 * (record.normal.y + 1.) * record.color.y,
            z: 0.5 * (record.normal.z + 1.) * record.color.z,
        };
        color * 255.9
    } else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        let r = 1. - 0.5 * t;
        let g = 1. - 0.3 * t;
        let b = 1.0;

        let col = vector::Vector{x: r, y: g, z: b};
        col * 255.9
    }
}

fn main() -> std::io::Result<()> {

    let sphere1 = canvas::Sphere{
        radius: 0.5,
        center: vector::Vector{x: 0., y: 0., z: -1.},
        color: vector::Vector{x: 0., y: 1., z: 0.}
    };

    let sphere2 = canvas::Sphere{
        radius: 100.,
        center: vector::Vector{x: 0., y: -100.5, z: -1.},
        color: vector::Vector{x: 0.1, y: 0.1, z: 0.1}
    };

    let world = vec![sphere1, sphere2];

    let nx = 600;
    let ny = 300;

    let mut output = String::from("P3\n");
    output.push_str(&format!("{:?} {:?}\n255\n", nx, ny).to_string());


    let lower_left = vector::Vector{x: -2., y: -1., z: -1.};
    let horizontal = vector::Vector{x: 4., y: 0., z: 0.};
    let vertical = vector::Vector{x: 0., y: 2., z: 0.};
    let origin = vector::Vector{x: 0., y: 0., z: 0.};


    for i_y in (0..ny).rev() {
        for i_x in 0..nx {
            let u = i_x as f64 / nx as f64;
            let v = i_y as f64 / ny as f64;
            let ray = ray::Ray{
                origin: origin,
                direction: lower_left + u * horizontal + v * vertical
            };

            let col = color(ray, &world);

            output.push_str(&format!("{:?} {:?} {:?}\n",
                                     col.x as i64,
                                     col.y as i64,
                                     col.z as i64)
                  .to_string());
        }
        // output.push_str("\n");
    }

    let mut file = File::create("image.ppm")?;
    file.write_all(output.as_bytes())?;
    Ok(())
}
