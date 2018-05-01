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
use hitable::{Hitable,HitRecord,Sphere};

fn hit_sphere(center:Vec3, radius:f64, r:&Ray) -> f64 {
	let oc = r.origin - center;
	let a = r.direction.dot(r.direction);
	let b = 2.0*oc.dot(r.direction);
	let c = oc.dot(oc) - radius*radius;
	let discriminant = b*b - 4.0*a*c;
	if (discriminant < 0.0) {
		return -1.0;
	}
	(-b-discriminant.sqrt()) / (2.0*a)
}

fn get_ray_color(r:Ray) -> Vec3 {
	let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, &r);
	if t>0.0 {
		let n = (r.point_at(t) -Vec3::new(0.0, 0.0, -1.0)).normalize();
		return 0.5*Vec3::new(n.x+1.0, n.y+1.0, n.z+1.0)
	}
	let unit_dir = &r.direction.normalize();
	let t = 0.5*(unit_dir.y + 1.0);
	(1.0-t)*Vec3::one() + t*Vec3::new(0.5, 0.7, 1.0)
}

fn main() {

	let image_width = 100;
	let image_height = 100;

	let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
	let horizontal = Vec3::new(4.0, 0.0, 0.0);
	let vertical = Vec3::new(0.0, 2.0, 0.0);
	let origin = Vec3::zero();

	let mut imgbuf = image::ImageBuffer::new(image_width, image_height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
		let u = x as f64 / image_width as f64;
		let v = y as f64 / image_height as f64;
		let r = Ray{
			origin: origin,
			direction: lower_left_corner + u*horizontal + v*vertical,
			time: 0.0
		};
		let col = get_ray_color(r);

    	*pixel = image::Rgb([
    		(255.0 * col.x) as u8, 
    		(255.0 * col.y) as u8, 
    		(255.0 * col.z) as u8
    	]);
	}

	image::ImageRgb8(imgbuf).save("test.png").expect("fuk");
}