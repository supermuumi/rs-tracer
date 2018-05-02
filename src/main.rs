extern crate image;
extern crate rand;
extern crate chrono;

#[macro_use]
extern crate structopt;

mod vector3;
mod ray;
mod camera;
mod aabb;
mod hitable;
mod material;

use chrono::prelude::*;
use structopt::StructOpt;

use vector3::Vec3;
use ray::Ray;
use camera::Camera;
use hitable::{Hitable,HitableList,Sphere};
use material::Material;

#[derive(StructOpt,Debug)]
#[structopt(name="raytracer")]
struct Options {
	#[structopt(short="w", help="output image width", default_value="200")]
	image_width: u32,

	#[structopt(short="h", help="output image height", default_value="200")]
	image_height: u32,

	#[structopt(short="s", help="num samples per pixel", default_value="10")]
	num_samples: u32,
}

fn get_ray_color(world:&HitableList, r:Ray, depth:u32) -> Vec3 {
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

	obj_list.push(Box::new(Sphere {
		center: Vec3::new(0.0,-1000.0,0.0),
		radius: 1000.0,
		material: Material::Lambertian{albedo: Vec3::new(0.5, 0.5, 0.5)},
	}));

	let n = 4;

	for a in -n..n { 
		for b in -n..n {

			let c = Vec3::new(a as f64 + 0.9 * rand::random::<f64>(), 0.2, b as f64 + 0.9 * rand::random::<f64>());
			let mat_rand = rand::random::<f64>();
			let mut mat = Material::Lambertian {
                            albedo: Vec3::new(
                                rand::random::<f64>() * rand::random::<f64>(),
                                rand::random::<f64>() * rand::random::<f64>(),
                                rand::random::<f64>() * rand::random::<f64>(),
                            ),
                        };
            if mat_rand < 0.3 {
            	mat = Material::Metal {
                            albedo: Vec3::new(
                                rand::random::<f64>() * rand::random::<f64>(),
                                rand::random::<f64>() * rand::random::<f64>(),
                                rand::random::<f64>() * rand::random::<f64>(),
                            ),
                            fuzziness: 0.2,
                        };
            }

          	obj_list.push(Box::new(
          		Sphere {
                        center: c,
                        radius: 0.2,
                        material: mat,
                }
            ));
		}
	}
	
	obj_list
}

fn main() {

	// parse command line
	let opt = Options::from_args();
	let image_width = opt.image_width;
	let image_height = opt.image_height;
	let num_samples = opt.num_samples;

	// create scene
	// TODO make this return a scene with camera settings etc. 
	let world = HitableList { list: create_scene() };
	
	// set camera
	let look_from = Vec3::new(-3.0, 1.0, 1.0); 
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
		for _n in 1..num_samples {
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
    	if n%image_width == 0 {
    		println!("{:.2}%", y as f64 * 100.0 / image_height as f64);
    	}
	}

	// write image
	let dt = Local::now();
	let fname = format!("out_{}.png", dt.format("%Y%m%d_%H%M%S").to_string());
	image::ImageRgb8(imgbuf).save(fname).expect("fuk");
}