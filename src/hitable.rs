use ray::Ray;
use vector3::Vec3;
use material::Material;
use aabb::AABox;

#[derive(Clone,Copy)]
pub struct HitRecord {
	pub dist: f64,
	pub point: Vec3, 
	pub normal: Vec3, 
	pub material: Material,
	pub u:f64,
	pub v:f64
}

/*

This works for editing function parameters

fn foo(x:&mut i32) {
	*x = 5;
}

fn main() {
	let mut x = 0;
	println!("{:?}", x);
	foo(&mut x);
	println!("{:?}", x);
}

*/

pub trait Hitable {
	fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> Option<HitRecord>;
	fn bounding_box(&self, t0:f64, t1:f64) -> Option<AABox>;
}

pub struct HitableList {
	pub list:Vec<Box<Hitable>>,
}

pub struct Sphere {
	pub center: Vec3,
	pub radius: f64,
	pub material: Material,
}

impl Hitable for Sphere {
	fn hit(&self, r: &Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
		let oc = r.origin - self.center;
		let a = r.direction.dot(r.direction);
		let b = oc.dot(r.direction);
		let c = oc.dot(oc) - self.radius*self.radius;
		let discriminant = b*b - a*c;

		if discriminant > 0.0 {
			let temp = (-b - discriminant.sqrt()) / a;
			if (temp > t_min && temp < t_max) {
				let p = r.point_at(temp);
				return Some(HitRecord {
					dist: temp,
					point: p,
					normal: (p - self.center) / self.radius,
					material: self.material,
					u: 0.0,
					v: 0.0,
				});
				//getSphereUV((rec.point - self.center) / self.radius, rec.u, rec.v);
			}
			let temp = (-b + discriminant.sqrt()) / a;
			if temp > t_min && temp < t_max {
				let p = r.point_at(temp);
				return Some(HitRecord {
					dist: temp,
					point: p,
					normal: (p - self.center) / self.radius,
					material: self.material,
					u: 0.0,
					v: 0.0,
				});
			}
		}
		None
	}

	fn bounding_box(&self, t0:f64, t1:f64) -> Option<AABox> {
		Some(AABox{
			min: self.center - Vec3::new(self.radius, self.radius, self.radius),
			max: self.center + Vec3::new(self.radius, self.radius, self.radius)
		})
	}
}

impl Hitable for HitableList {
    fn hit(&self, r: &Ray, t_min:f64, t_max:f64) -> Option<HitRecord> {
        let mut best = None;
        for child in &self.list {
            if let Some(hit) = child.hit(&r, t_min, t_max) {
                match best {
                    None => best = Some(hit),
                    Some(prev) => if hit.dist < prev.dist {
                        best = Some(hit)
                    }
                }
            }
        }
        best
    }

	fn bounding_box(&self, t0:f64, t1:f64) -> Option<AABox> {
		None
	}
}