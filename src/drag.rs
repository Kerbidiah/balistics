use glam::DVec3;

#[derive(Debug, Copy, Clone)]
pub struct Drag {
	pub area: f64,
	pub coef_drag: f64,
	pub density: f64 // kg/m^3, air is 1.225
}

impl Drag {
	pub fn drag_force(&self, vel: DVec3) -> DVec3 {
		let drag_mag = -0.5
			* self.area
			* self.coef_drag
			* self.density
			* vel.length_squared();

		vel.normalize_or_zero() * drag_mag // combines direction w/ magnitude 
	}
}
