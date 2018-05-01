extern crate image;
extern crate rand;

mod vector3;
mod ray;
mod camera;
mod aabb;
mod hitable;
mod material;

use std::fs::File;

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
	let mut v: Vec<Box<Hitable>> = Vec::with_capacity(500);

	v.push(Box::new(Sphere {
		center: Vec3::new(-1.0,0.0,-1.0),
		radius: 0.5,
		material: Material::Lambertian{albedo: Vec3::new(0.8,0.0,0.0)},
	}));
	v.push(Box::new(Sphere {
		center: Vec3::new(1.0,0.0,-1.0),
		radius: 0.5,
		material: Material::Lambertian{albedo: Vec3::new(0.8,0.0,0.0)},
	}));
	v
}

fn main() {

	let image_width = 100;
	let image_height = 100;
	let num_samples = 100;

	let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
	let horizontal = Vec3::new(4.0, 0.0, 0.0);
	let vertical = Vec3::new(0.0, 2.0, 0.0);
	let origin = Vec3::zero();

	let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

	let world = HitableList { list: create_scene() };

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		let u = x as f64 / image_width as f64;
		let v = y as f64 / image_height as f64;
		let r = Ray{
			origin: origin,
			direction: lower_left_corner + u*horizontal + v*vertical,
			time: 0.0
		};

		let mut col = Vec3::zero();
		for n in 1..num_samples {
			col = col+get_ray_color(&world, r, 0);
		}
		col = col/num_samples as f64;
		col = Vec3::new(col.x.sqrt(), col.y.sqrt(), col.z.sqrt());

    	*pixel = image::Rgb([
    		(255.0 * col.x) as u8, 
    		(255.0 * col.y) as u8, 
    		(255.0 * col.z) as u8
    	]);
	}

	image::ImageRgb8(imgbuf).save("test.png").expect("fuk");
}