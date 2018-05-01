use ray::Ray;
use hitable::HitRecord;
use vector3;
use vector3::Vec3;

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

#[derive(Clone, Copy)]
pub enum Material {
	Lambertian { 
		albedo: Vec3, 
	},

	Metal {
		albedo: Vec3,
		fuzziness: f64,
	},
	
	Dielectric {
		ref_idx: f64,
	}

}

impl Material {
	fn scatter_lambertian(albedo:Vec3, _:&Ray, rec:&HitRecord) -> Option<(Vec3, Ray)> {
		let target = rec.point + rec.normal + vector3::get_random_in_unit_sphere();
		Some((albedo, Ray { origin: rec.point, direction: target-rec.point, time:0.0 }))
	}

	fn scatter_metal(albedo:Vec3, fuzziness:f64, _:&Ray, rec:&HitRecord) -> Option<(Vec3, Ray)> {
		// TODO
		None
	}

	fn scatter_dielectric() -> Option<(Vec3, Ray)> {
		// TODO
		None
	}

	fn scatter(&self, ray_in:&Ray, rec:&mut HitRecord, scattered:&mut Ray) -> Option<(Vec3, Ray)> {
		match *self {
			Material::Lambertian{albedo} => {
				Material::scatter_lambertian(albedo, ray_in, rec)
			}
			Material::Metal{albedo, fuzziness} => {
				None //Material::scatter_metal(albedo, fuzziness, ray_in, rec)
			}
			Material::Dielectric{ref_idx} => {
				None //Material::scatter_dielectric(ref_idx, ray_in, rec)
			}
		}
	}
}
