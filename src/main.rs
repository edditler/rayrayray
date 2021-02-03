use std::fs::File;
use std::io::prelude::*;
use rand;

extern crate pbr;
use pbr::ProgressBar;

mod vector;
mod ray;
mod canvas;
mod camera;
mod materials;

fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value > max { return max; }
    if value < min { return min; }
    value
}

fn random_point_in_unit_sphere() -> vector::Vector {
    let mut point = vector::Vector{x: 1., y: 1., z: 1.};
    while point.norm2() >= 1. {
        point = vector::Vector{x: rand::random(),
                               y: rand::random(),
                               z: rand::random()};
        point = point * 2. - 1.;
    }
    point
}


fn color_ray(ray: ray::Ray, world: &Vec<canvas::Sphere>, depth: usize) -> vector::Vector {
    if depth > 50 {
        return vector::Vector{x: 0.0, y: 0.0, z: 0.0};
    }

    let mut closest_so_far = 10e100;
    let mut hit_anything = false;
    let mut normal = vector::Vector{x: 0.0, y: 0.0, z: 0.0};
    let mut point = vector::Vector{x: 0.0, y: 0.0, z: 0.0};
    let mut material = materials::Material::Diffuse{
        albedo: vector::Vector{x: 1., y: 1., z: 1.}
    };

    // Check for every sphere whether we hit it
    for sphere in world.iter() {
        // We want the smallest param possible!
        let (param, hit_this_time) = sphere.hit_ray(ray, 0.0000001, closest_so_far);
        if hit_this_time {
            point = ray.point_at_parameter(param);
            normal = (point - sphere.center) / sphere.radius;
            material = sphere.material;
            hit_anything = true;
            closest_so_far = param;
        }
    }

    if hit_anything {
        match material {
            materials::Material::FancyNormals{} => {
                return vector::Vector{
                    x: 127.5 * (normal.x + 1.),
                    y: 127.5 * (normal.y + 1.),
                    z: 127.5 * (normal.z + 1.),
                }
            },
            materials::Material::ConstantColor{color} => {
                return color
            },
            materials::Material::Diffuse{albedo} => {
                let target = normal + random_point_in_unit_sphere();
                let scattered_ray = ray::Ray{origin: point, direction: target};

                let col = color_ray(scattered_ray, &world, depth+1);
                return vector::Vector{
                    x: col.x * albedo.x,
                    y: col.y * albedo.y,
                    z: col.z * albedo.z,
                }
            },
            materials::Material::Metal{albedo, fuzz} => {
                let mut reflection_direction = ray.direction;
                reflection_direction -= 2. * (ray.direction * normal) * normal;
                reflection_direction += random_point_in_unit_sphere() * fuzz;

                let mut col = vector::Vector{x: 0.0, y: 0.0, z: 0.0};

                if reflection_direction.unit_vector() * normal > 0. {
                    let scattered = ray::Ray{
                        origin: point,
                        direction: reflection_direction.unit_vector()
                    };

                    col = color_ray(scattered, &world, depth+1);
                }

                return vector::Vector{
                    x: col.x * albedo.x,
                    y: col.y * albedo.y,
                    z: col.z * albedo.z,
                }
            },
            materials::Material::Dielectric{refractive_index} => {
                let mut reflection_direction = ray.direction;
                reflection_direction -= 2. * (ray.direction * normal) * normal;

                let outward_normal;
                let refractive_ratio;

                if reflection_direction.unit_vector() * normal > 0. {
                    outward_normal = -normal;
                    refractive_ratio = refractive_index
                } else {
                    outward_normal = normal;
                    refractive_ratio = 1./refractive_index
                }

                let unit_direction = ray.direction.unit_vector();
                let dt = unit_direction * outward_normal;
                let discriminant = 1. - refractive_ratio*refractive_ratio * (1. - dt*dt);

                let scattered;
                if discriminant > 0. {
                    // The ray is refracted
                    let mut refracted = refractive_ratio * (unit_direction - outward_normal*dt);
                    refracted = refracted - refractive_index * discriminant.sqrt();
                    scattered = ray::Ray{origin: point, direction: refracted}
                } else {
                    // Total reflection!
                    scattered = ray::Ray{origin: point, direction: reflection_direction}
                }

                let col = color_ray(scattered, &world, depth+1);
                return col
            }
        };
    } else {
        // The sky :)
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);

        let r = 1. - 0.5 * t;
        let g = 1. - 0.3 * t;
        let b = 1.0;

        let color = vector::Vector{x: r, y: g, z: b};
        color * 255.9
    }
}


// Main routine
fn main() -> std::io::Result<()> {
    // auto material_left   = make_shared<metal>(color(0.8, 0.8, 0.8));
    // auto material_right  = make_shared<metal>(color(0.8, 0.6, 0.2));

    // world.add(make_shared<sphere>(point3(-1.0,    0.0, -1.0),   0.5, material_left));
    // world.add(make_shared<sphere>(point3( 1.0,    0.0, -1.0),   0.5, material_right));

    let ground_sphere = canvas::Sphere {
        radius: 100.,
        center: vector::Vector{x: 0., y: -100.5, z: -1.},
        material: materials::Material::Diffuse {
            albedo: vector::Vector{x: 0.8, y: 0.8, z: 0.0}
        }
    };

    let sphere1 = canvas::Sphere{
        radius: 0.5,
        center: vector::Vector{x: 0., y: 0., z: -1.},
        material: materials::Material::Diffuse {
            albedo: vector::Vector{x: 0.7, y: 0.3, z: 0.3}
        }
    };

    let sphere2 = canvas::Sphere{
        radius: 0.5,
        center: vector::Vector{x: -1.0, y: 0., z: -1.0},
        material: materials::Material::Metal {
            albedo: vector::Vector{x: 0.8, y: 0.8, z: 0.8},
            fuzz: 0.3
        }
    };

    let sphere3 = canvas::Sphere{
        radius: 0.5,
        center: vector::Vector{x: 1.0, y: 0., z: -1.0},
        material: materials::Material::Metal {
            albedo: vector::Vector{x: 0.8, y: 0.6, z: 0.2},
            fuzz: 1.0
        }
    };


    let world = vec![ground_sphere, sphere1, sphere2, sphere3];

    // We define our resulting image...
    let aspect_ratio = 16./9.;
    let image_width = 1000;
    let image_height = (image_width as f64/aspect_ratio) as u64;
    let n_samples = 20;
    // ... and the virtual viewport
    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.;

    let origin = vector::Vector{x: 0., y: 0., z: 0.};
    let horizontal = vector::Vector{x: viewport_width, y: 0., z: 0.};
    let vertical =vector::Vector{x: 0., y: viewport_height, z: 0.};
    let cam = camera::Camera{
        origin: origin,
        horizontal: horizontal,
        vertical: vertical,
        lower_left: origin - (horizontal+vertical)/2. - vector::Vector{x: 0., y: 0., z: focal_length},
    };

    let mut output = String::from("P3\n");
    output.push_str(&format!("{:?} {:?}\n255\n", image_width, image_height).to_string());

    // For the whole canvas...
    let mut pb = ProgressBar::new(image_height);
    for i_y in (0..image_height).rev() {
        pb.inc();
        for i_x in 0..image_width {
            // ... define a ray by its origin and direction.
            let mut col = vector::Vector{x: 0., y: 0., z: 0.};
            // Including anti-aliasing
            for _i_sample in 0..n_samples {
                let u = (i_x as f64 + rand::random::<f64>()) / (image_width-1) as f64;
                let v = (i_y as f64 + rand::random::<f64>()) / (image_height-1) as f64;
                let ray = cam.get_ray(u, v);

                // Define the color of the ray by its intersections with the world.
                col += color_ray(ray, &world, 0);
            }

            col = col/(n_samples as f64);
            // col = col.sqrt();  // Gamma = 2

            // Each line is a pixel.
            output.push_str(&format!("{:?} {:?} {:?}\n",
                                     clamp(col.x, 0., 255.) as i64,
                                     clamp(col.y, 0., 255.) as i64,
                                     clamp(col.z, 0., 255.) as i64)
                  .to_string());
        }
        // output.push_str("\n");
    }
    pb.finish();


    let mut file = File::create("image.ppm")?;
    file.write_all(output.as_bytes())?;
    Ok(())
}
