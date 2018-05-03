use vector3::Vec3;

#[derive(Clone, Copy, Debug)]
pub struct Ray {
	pub origin:Vec3,
	pub direction:Vec3,
	pub time:f32,
}

impl Ray {
	pub fn point_at(self,t:f32) -> Vec3 {
		self.origin + t*self.direction
	}
}