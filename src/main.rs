extern crate image;
extern crate rand;
extern crate chrono;

mod vector3;
mod ray;
mod camera;
mod aabb;
mod hitable;
mod material;

use rand::random;
use chrono::prelude::*;

use vector3::Vec3;
use ray::Ray;
use camera::Camera;
use hitable::{Hitable,HitableList,HitRecord,Sphere};
use material::Material;

fn get_ray_color(world:&HitableList, r:Ray, depth:u32) -> Vec3 {
	let s = Sphere {
		center: Vec3::new(0.0,0.0,-1.0),
		radius: 0.5,
		material: Material::Lambertian{albedo: Vec3::new(0.8,0.0,0.0)},
	};

	if let Some(ray_hit) = world.hit(&r, 0.0001, std::f64::MAX) {
		if let Some((attenuation, scattered)) = ray_hit.material.scatter(&r, &ray_hit) {
			return attenuation * get_ray_color(world, scattered, depth + 1);
		}
		return Vec3::zero()
	}

	let unit_dir = &r.direction.normalize();
	let t = 0.5*(unit_dir.y + 1.0);
	(1.0-t)*Vec3::one() + t*Vec3::new(0.5, 0.7, 1.0)
}

fn create_scene() -> Vec<Box<Hitable>> {
	let mut obj_list: Vec<Box<Hitable>> = Vec::with_capacity(500);

	obj_list.push(Box::new(
		Sphere {
            center: Vec3::new(0.0, -1000.0, 0.0),
            radius: 1000.0,
            material: Material::Lambertian {
                albedo: Vec3::new(0.5, 0.5, 0.5),
            },
        })
   	);

	obj_list.push(Box::new(Sphere {
		center: Vec3::new(0.0,-1000.0,0.0),
		radius: 1000.0,
		material: Material::Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)},
	}));

	let n = 4;

	for a in -n..n { 
		for b in -n..n {
          	obj_list.push(Box::new(
          		Sphere {
                        center: Vec3::new(
				                    a as f64 + 0.9 * rand::random::<f64>(),
				                    0.2,
				                    b as f64 + 0.9 * rand::random::<f64>(),
				                ),
                        radius: 0.2,
                        material: Material::Lambertian {
                            albedo: Vec3::new(
                                rand::random::<f64>() * rand::random::<f64>(),
                                rand::random::<f64>() * rand::random::<f64>(),
                                rand::random::<f64>() * rand::random::<f64>(),
                            ),
                        },
                }
            ));
		}
	}
	
	obj_list
}

fn main() {

	let image_width = 200;
	let image_height = 200;
	let num_samples = 10;

	let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
	let horizontal = Vec3::new(4.0, 0.0, 0.0);
	let vertical = Vec3::new(0.0, 2.0, 0.0);
	let origin = Vec3::zero();


	// create scene
	let world = HitableList { list: create_scene() };
	
	// set camera
	let look_from = Vec3::new(-3.0, 2.0, 1.0); 
	let look_at = Vec3::new(0.0, 0.0, -1.0);
	let cam = Camera::new(
		look_from, look_at,
		Vec3::new(0.0, -1.0, 0.0), 
		90.0, image_width as f64 / image_height as f64,
		0.0, (look_from-look_at).length(),
		0.0, 1.0
	);

	// let's off-road!
	let mut n:u32 = 0;
	let mut imgbuf = image::ImageBuffer::new(image_width, image_height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {

		let mut col = Vec3::zero();
		for n in 1..num_samples {
			let u = (x as f64 + rand::random::<f64>()) / image_width as f64;
			let v = (y as f64 + rand::random::<f64>()) / image_height as f64;
			let r = cam.get_ray(u, v);
			col = col + get_ray_color(&world, r, 0);
		}
		col = col/num_samples as f64;
		col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

    	*pixel = image::Rgb([
    		(255.0 * col.x) as u8, 
    		(255.0 * col.y) as u8, 
    		(255.0 * col.z) as u8
    	]);
    	n = n+1;
    	if n%100 == 0 {
    		println!("{:.2}%", y as f64 * 100.0 /image_height as f64);
    	}
	}

	let dt = Local::now();
	let fname = format!("out_{}.png", dt.format("%Y%m%d_%H%M%S").to_string());
	image::ImageRgb8(imgbuf).save(fname).expect("fuk");
}