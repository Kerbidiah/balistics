use ::glam::DVec3;
use macroquad::prelude::*;

mod projectile;
mod drag;
mod controls;

#[allow(dead_code)]
const T_MAX: f64 = 20.0;
const RADIUS: f32 = 0.25;

fn conf() -> Conf {
    Conf {
        window_title: String::from("Balistics"),
        high_dpi: true,
		sample_count: 4, // anti-alliasing
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
	// im going to consider y to be up/down
	let grav: DVec3 = DVec3::new(0.0, -9.81, 0.0);

	let mut shell_a = projectile::Projectile {
		pos: DVec3::ZERO,
		vel: DVec3::new(0.0, 25.0, 5.0),
		mass: 1.0,
		grav: grav,
		accel: DVec3::ZERO,
		drag: Some(drag::Drag {
			area: 1.0,
			coef_drag: 1.,
			density: 1.225
		})
	};
	
	let mut shell_b = shell_a;
	shell_b.drag = None;

	#[allow(unused_variables)]
	let mut time: f64 = 0.0;
	let mut dt;

	let mut camera_stuff = controls::CameraStuff::new(
		vec3(-15., 5., 0.), // position
		vec3(1., 0., 0.), // direction
	);

	loop {
		dt = get_frame_time() as f64;

		shell_a.step(dt);
		shell_b.step(dt);

		if is_key_pressed(KeyCode::P) {
			info!("{}\n{}", shell_a.pos, shell_b.pos);
		}
		if is_key_pressed(KeyCode::K) { // escape hatch
			
			// reset the mouse
			camera_stuff.is_mouse_captured = false;
			camera_stuff.mouse_trap();
			break // end program
		}
		
		clear_background(BLACK);
		
		// move camera with wasd and qe
		camera_stuff.mouse_trap();
		camera_stuff.mouse_look();
		camera_stuff.wasd();
		
		// go into 3d and place camera
		camera_stuff.place();
		
		draw_grid(40, 1.0, YELLOW, WHITE);
		draw_sphere_wires(vec3(0., 0., 0.), RADIUS, None, WHITE);
		draw_sphere(shell_a.pos_macroq(), RADIUS, None, YELLOW);
		draw_sphere(shell_b.pos_macroq(), RADIUS, None, ORANGE);

		set_default_camera(); // go back to 2d screen space to draw text
		draw_text( // camera position
			&format!("{}", camera_stuff.position),
			20.0, 20.0, 20.0, WHITE
		);

		draw_text( // mouse trap status
			&format!("{}", camera_stuff.is_mouse_captured),
			20.0, 40.0, 20.0, WHITE
		);
		draw_text( // mouse position
			&format!("{}", camera_stuff.forward),
			70.0, 40.0, 20.0, WHITE
		);

		time += dt;
		next_frame().await
	}
}
