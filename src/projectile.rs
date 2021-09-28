use glam::DVec3;
use crate::drag;

#[derive(Debug, Copy, Clone)]
pub struct Projectile {
	pub pos: DVec3,
	pub vel: DVec3,
	pub mass: f64,
	pub grav: DVec3, // if you don't want gravity, just make it 0
	pub accel: DVec3, // acceleration that has yet to be applied
	pub drag: Option<drag::Drag> // drag is optional
}

/*trait Aero_Drag {
	drag: Option<Drag>
}*/

impl Projectile {
	#[inline]
	pub fn force(&mut self, force: DVec3) {
		self.accel += force / self.mass;
	}

	#[inline] // should it just return 0 if no drag???
	pub fn drag_force(&self) -> Result<DVec3, ()> {
		if let Some(drag) = &self.drag {
			Ok(drag.drag_force(self.vel))
		} else {
			Err(())
		}
	}

	#[inline]
	fn apply_drag(&mut self) {
		self.force(
			self.drag_force()
			.unwrap_or_default()
		);
	}

	#[inline]
	pub fn pos_macroq(&self) -> macroquad::math::Vec3 {
		macroquad::math::vec3(
			self.pos.x as f32,
			self.pos.y as f32,
			self.pos.z as f32
		)
	}

	#[inline] // should it be inlined???
	pub fn step(&mut self, dt: f64) {

		self.apply_drag(); // drag
		self.accel += self.grav; // gravity

		self.vel += self.accel * dt; // change velocity
		self.pos += self.vel * dt; // change position

		self.accel = DVec3::ZERO; // wipe out all accelerations
	}
}
