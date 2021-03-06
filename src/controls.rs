use macroquad::prelude::*;

const MOVE_SPEED: f32 = 5.0; // m/s
const LOOK_SPEED: f32 = 0.1;
const GLOBAL_UP: Vec3 = Vec3::Y; // jHat

#[derive(Debug)]
pub struct CameraStuff {
	pub position: Vec3,
	pub forward: Vec3,
	up: Vec3,
	right: Vec3,
	pub last_mouse_pos: Vec2,
	pub is_mouse_captured: bool
}

impl CameraStuff {
	pub fn new(position: Vec3, forward: Vec3) -> CameraStuff {

		// use cross product to find vector to the right
		let right = forward.cross(GLOBAL_UP).normalize();

		// use corss prodcut to find vector pointing up from the camera's perspective
		let up = right.cross(forward).normalize();

		let mut ans = CameraStuff {
			position,
			forward, // DON'T start w/ camera looking up/down
			up,
			right,
			last_mouse_pos: Vec2::ZERO,
			is_mouse_captured: false
		};

		ans.normalize();
		ans
	}

	#[inline]
	pub fn place(&self) {
		set_camera(&Camera3D {
			position: self.position,
            up: self.up,
			target: self.position + self.forward,
            ..Default::default()
        });
	}

	#[inline]
	pub fn mouse_look(&mut self) {
		
		let m_pos = if self.is_mouse_captured {
			mouse_position().into()
		} else {
			Vec2::ZERO
		};

		let look_speed: f32 = LOOK_SPEED * get_frame_time();
		let delta: Vec2 = (m_pos - self.last_mouse_pos) * look_speed;

		// all angles here are in radians
		// let rho = self.forward.length();
		// let cyl_radius = vec2(self.forward.z, self.forward.x).length();

		let mut azimuth = self.forward.z.atan2(self.forward.x); //+ std::f32::consts::PI
		let mut elevation = std::f32::consts::FRAC_PI_2 - self.forward.angle_between(GLOBAL_UP);

		azimuth += delta.x;
		elevation -= delta.y;

		elevation = elevation.clamp(-179f32.to_radians(), 179f32.to_radians()); // make sure we don't look straight down

		self.forward.x = azimuth.cos() * elevation.cos();
		self.forward.y = elevation.sin();
		self.forward.z = azimuth.sin() * elevation.cos();
		
		self.forward = self.forward.normalize();
		self.update_all_vecs();

		// warn!("{}", self.forward.length());

		self.last_mouse_pos = m_pos;
		
	}
	
	#[inline]
	pub fn wasd(&mut self) {
		let move_size: f32 = MOVE_SPEED * get_frame_time();
		let mut delta = Vec3::ZERO;

		// forward/backward
		if is_key_down(KeyCode::W) {
			delta += self.forward;
		}
		if is_key_down(KeyCode::S) {
			delta -= self.forward;
		}

		// up/down
		if is_key_down(KeyCode::Space) {
			delta += self.up;
		}
		if is_key_down(KeyCode::LeftShift) {
			delta -= self.up;
		}

		// left/right
		if is_key_down(KeyCode::D) {
			delta += self.right;
		}
		if is_key_down(KeyCode::A) {
			delta -= self.right;
		}

		delta.normalize();
		delta *= move_size;

		self.position += delta;
	}

}


// vector stuff
impl CameraStuff {
	#[inline]
	fn normalize(&mut self) {
		self.forward = self.forward.normalize();
		self.up = self.up.normalize();
		self.right = self.right.normalize();
	}

	#[inline(always)]
	fn update_right_vec(&mut self) {
		self.right = self.forward.cross(GLOBAL_UP).normalize();
	}

	#[inline(always)]
	fn update_up_vec(&mut self) {
		self.up = self.right.cross(self.forward).normalize();
	}

	#[inline(always)]
	fn update_all_vecs(&mut self) {
		self.update_right_vec();
		self.update_up_vec();
	}
}

// fixme: I don;t think this works????
// mouse capture stuff (makes mouse disapear and control the camera)
impl CameraStuff {
	#[inline(always)]
	pub fn toggle_mouse(&mut self) {
		self.is_mouse_captured = !self.is_mouse_captured;
		if self.is_mouse_captured == false {
			self.last_mouse_pos = Vec2::ZERO;
		}
	}

	//#[inline(always)]
	pub fn mouse_trap(&mut self){
		/* FIXME: doesn't allow browser to steal cursor back with escape key correctly
			if is_key_pressed(KeyCode::Tab) || (
			self.is_mouse_captured &&
			is_key_pressed(KeyCode::Escape) {
				...
			}
		)

		*/
		if is_key_pressed(KeyCode::Tab){ 
			self.toggle_mouse();
		}

		match self.is_mouse_captured {
			true => {
				show_mouse(false);
				set_cursor_grab(true);
			},
			false => {
				show_mouse(true);
				set_cursor_grab(false);
			}
		}
	}
}