extern crate rand;

use vector3;
use vector3::Vec3;
use ray::Ray;
use std;

#[derive(Debug)]
pub struct Camera {
	pub origin:Vec3,
	pub lower_left_corner:Vec3,
	pub horizontal:Vec3,
	pub vertical:Vec3,
	pub u:Vec3, 
	pub v:Vec3, 
	pub w:Vec3,	
	pub lens_radius:f64, 
	pub shutter_open:f64, 
	pub shutter_close:f64, 
}

impl Camera {
	pub fn new(look_from:Vec3, look_at:Vec3, up:Vec3, 
		   v_fov:f64, aspect:f64, 
		   aperture:f64, focal_dist:f64,
		   t0:f64, t1:f64) -> Camera {


		let theta = v_fov * std::f64::consts::PI / 180.0;
		let half_height = (theta / 2.0).tan();
		let half_width = half_height * aspect;

		let w = (look_from - look_at).normalize();
		let u = up.cross(w).normalize();
		let v = w.cross(u);

		Camera{
			origin: look_from,
			lower_left_corner: look_from - half_width * focal_dist*u - half_height * focal_dist*v - focal_dist * w,
			horizontal: 2.0 * half_width * focal_dist*u,
			vertical: 2.0 * half_height * focal_dist*v,
			lens_radius: aperture / 2.0,
			shutter_open: t0,
			shutter_close: t1,
			u: u,
			v: v,
			w: w,
		}
	}

	pub fn get_ray(&self, s:f64, t:f64) -> Ray {
		let rd = self.lens_radius*vector3::get_random_in_unit_disc();
		let offset = self.u*rd.x + self.v*rd.y;
		let time = self.shutter_open + rand::random::<f64>()*(self.shutter_close-self.shutter_open);
		Ray {
			origin: self.origin + offset, 
			direction: self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin - offset, 
			time: time
		}
	}
}