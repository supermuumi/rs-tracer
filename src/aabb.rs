use vector3::Vec3;
use ray::Ray;

pub struct AABox {
	pub min:Vec3,
	pub max:Vec3
}

impl AABox {
	#[allow(dead_code)]
	fn hit(self, r:Ray, t_min_orig:f64, t_max_orig:f64) -> bool  {
		let inv_d = 1.0 / r.direction.x;
		let mut t0 = (self.min.x - r.origin.x) * inv_d;
		let mut t1 = (self.max.x - r.origin.x) * inv_d;
		let mut t_min = t_min_orig;
		let mut t_max = t_max_orig;
		if inv_d < 0.0 {
			let temp = t0;
			t0 = t1;
			t1 = temp;
		}
		if t0 > t_min {
			t_min = t0;
		}
		if t1 < t_max {
			t_max = t1;
		}
		if t_max <= t_min {
			return false;
		}

		let inv_d = 1.0 / r.direction.y;
		t0 = (self.min.y - r.origin.y) * inv_d;
		t1 = (self.max.y - r.origin.y) * inv_d;
		if inv_d < 0.0 {
			let temp = t0;
			t0 = t1;
			t1 = temp;
		}
		if t0 > t_min {
			t_min = t0;
		}
		if t1 < t_max {
			t_max = t1;
		}
		if t_max <= t_min {
			return false;
		}

		let inv_d = 1.0 / r.direction.z;
		t0 = (self.min.z - r.origin.z) * inv_d;
		t1 = (self.max.z - r.origin.z) * inv_d;
		if inv_d < 0.0 {
			let temp = t0;
			t0 = t1;
			t1 = temp;
		}
		if t0 > t_min {
			t_min = t0;
		}
		if t1 < t_max { 
			t_max = t1;
		}
		if t_max <= t_min {
			return false;
		}

		true
	}
}

#[allow(dead_code)]
pub fn combine_AABB(a:AABox, b:AABox) -> AABox {
	let small_box = Vec3 {
		x: a.min.x.min(b.min.x),
		y: a.min.y.min(b.min.y),
		z: a.min.z.min(b.min.z)};
	let large_box = Vec3 {
		x: a.max.x.max(b.max.x),
		y: a.max.y.max(b.max.y),
		z: a.max.z.max(b.max.z)};

	AABox{min:small_box, max:large_box}
}