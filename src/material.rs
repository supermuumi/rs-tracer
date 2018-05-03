use ray::Ray;
use hitable::HitRecord;
use vector3;
use vector3::Vec3;
use rand::Rng;

// thanks to Cam Hart for pointing out an easy way to do this, kept banging my head against the wall trying to do it the C++ way
#[derive(Clone, Copy)]
pub enum Material {
	Lambertian { 
		albedo: Vec3, 
	},

	Metal {
		albedo: Vec3,
		fuzziness: f32,
	},
	
	Dielectric {
		ref_idx: f32,
	}

}

impl Material {
	fn scatter_lambertian(albedo:Vec3, _:&Ray, rec:&HitRecord, rng:&mut Rng) -> Option<(Vec3, Ray)> {
		let target = rec.point + rec.normal + vector3::get_random_in_unit_sphere(rng);
		Some((albedo, Ray { origin: rec.point, direction: target-rec.point, time:0.0 }))
	}

	fn scatter_metal(albedo:Vec3, fuzziness:f32, ray_in:&Ray, rec:&HitRecord, rng:&mut Rng) -> Option<(Vec3, Ray)> {
		let reflected = ray_in.direction.normalize().reflect(rec.normal);
		let scattered = Ray {
			origin: rec.point, 
			direction: reflected + fuzziness * vector3::get_random_in_unit_sphere(rng), 
			time: ray_in.time
		};

		if scattered.direction.dot(rec.normal) > 0.0 {
			return Some((albedo, scattered));
		}
		None
	}

	fn scatter_dielectric() -> Option<(Vec3, Ray)> {
		// TODO
		None
	}

	pub fn scatter(&self, ray_in:&Ray, rec:&HitRecord, rng:&mut Rng) -> Option<(Vec3, Ray)> {
		match *self {
			Material::Lambertian{albedo} => {
				Material::scatter_lambertian(albedo, ray_in, rec, rng)
			}
			Material::Metal{albedo, fuzziness} => {
				Material::scatter_metal(albedo, fuzziness, ray_in, rec, rng)
			}
			Material::Dielectric{ref_idx} => {
				None //Material::scatter_dielectric(ref_idx, ray_in, rec)
			}
		}
	}
}
