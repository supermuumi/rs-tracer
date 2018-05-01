
pub trait Texture {
	fn value(u:f64, v:f64, p:Vec3) -> Vec3;
}

struct ConstantTexture {
	color: Vec3;
}
impl Texture for ConstantTexture {
	fn value(u:f64, v:f64, p:Vec3) -> Vec3 {
		color
	}
}