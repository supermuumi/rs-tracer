extern crate rand;

use std::ops::{Add,Sub,Mul,Div,Neg};
use std::fmt;
use rand::Rng;

#[derive(Debug,Clone,Copy)]
pub struct Vec3 {
	pub x:f32,
	pub y:f32,
	pub z:f32
}

impl Vec3 {
	#[allow(dead_code)]
	pub fn new(_x:f32, _y:f32, _z:f32) -> Vec3 {
		Vec3{x:_x, y:_y, z:_z}
	}

	#[allow(dead_code)]
	pub fn zero() -> Vec3 {
		Vec3{x:0.0, y:0.0, z:0.0}
	}

	#[allow(dead_code)]
	pub fn one() -> Vec3 {
		Vec3{x:1.0, y:1.0, z:1.0}
	}

	#[allow(dead_code)]
	// TODO make this use rng
	pub fn random() -> Vec3 {
		Vec3 {
			x: rand::random::<f32>(), 
			y: rand::random::<f32>(),
			z: rand::random::<f32>()
		}
	}

	#[allow(dead_code)]
	pub fn length_sq(self) -> f32 {
		self.x*self.x + self.y*self.y + self.z*self.z
	}

	#[allow(dead_code)]
	pub fn length(self) -> f32 {
		self.length_sq().sqrt()
	}

	#[allow(dead_code)]
	pub fn normalize(self) -> Vec3 {
		let n = self.length();
		(self / n)
	}

	pub fn dot(self, v2:Vec3) -> f32 {
		self.x*v2.x + self.y*v2.y + self.z*v2.z
	}

	pub fn cross(self, v2:Vec3) -> Vec3 {
		Vec3 {
			x:   self.y*v2.z - self.z*v2.y,
			y: -(self.x*v2.z - self.z*v2.x),
			z:   self.x*v2.y - self.y*v2.x,
		}
	}

	pub fn reflect(self, n:Vec3) -> Vec3 {
		self - 2.0 * self.dot(n) * n
	}

	pub fn refract(self, n:Vec3, ni_over_nt:f32) -> (bool,Vec3) {
		let uv = self.normalize();
		let dt = uv.dot(n);
		let discriminant = 1.0 - ni_over_nt*ni_over_nt*(1.0 - dt*dt);		
		if discriminant > 0.0 {
			return (true, ni_over_nt*(uv - n*dt) - n*discriminant.sqrt());
		}
		(false,Vec3::zero())
	}
}

impl fmt::Display for Vec3 {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {} {})", self.x, self.y, self.z)
    }
}

impl Add<Vec3> for Vec3 {
	type Output=Vec3;

	fn add(self, v:Vec3) -> Vec3 {
		Vec3{
			x:self.x + v.x,
			y:self.y + v.y,
			z:self.z + v.z
		}
	}
}

impl Sub<Vec3> for Vec3 {
	type Output=Vec3;

	fn sub(self, v:Vec3) -> Vec3 {
		Vec3{
			x:self.x - v.x,
			y:self.y - v.y,
			z:self.z - v.z
		}
	}
}

impl Mul<f32> for Vec3 {
	type Output=Vec3;

	fn mul(self, t:f32) -> Vec3 {
		Vec3{x:self.x*t, y:self.y*t, z:self.z*t}
	}
}

impl Mul<Vec3> for Vec3 {
	type Output=Vec3;

	fn mul(self, v:Vec3) -> Vec3 {
		Vec3{x:self.x*v.x, y:self.y*v.y, z:self.z*v.z}
	}
}

impl Mul<Vec3> for f32 {
	type Output=Vec3;

	fn mul(self, v:Vec3) -> Vec3 {
		Vec3{x:self*v.x, y:self*v.y, z:self*v.z}
	}	
}

impl Div<f32> for Vec3 {
	type Output=Vec3;

	fn div(self, t:f32) -> Vec3 {
		Vec3{x:self.x/t, y:self.y/t, z:self.z/t}
	}
}

impl Neg for Vec3 {
	type Output=Vec3;

	fn neg(self) -> Vec3 {
		Vec3{x:-self.x, y:-self.y, z:-self.z}
	}
}

pub fn get_random_in_unit_sphere(rng: &mut Rng) -> Vec3 {
	loop {
		let p = 2.0*Vec3::new(rng.next_f32(), rng.next_f32(), rng.next_f32()) - Vec3::one();
		if p.length_sq() < 1.0 {
			return p;
		}
	}
}

pub fn get_random_in_unit_disc(rng: &mut Rng) -> Vec3 {
	loop {
		let p = 2.0*Vec3::new(rng.next_f32(), rng.next_f32(), 0.0) - Vec3::new(1.0, 1.0, 0.0);
		if p.dot(p) < 1.0 {
			return p;
		}
	}
}