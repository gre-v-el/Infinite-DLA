use std::f32::consts::PI;

use macroquad::prelude::*;

struct Particle {
	pos: Vec2,
	vel: Vec2,
	r: f32,
}

#[macroquad::main("infinite DLA")]
async fn main() {
	let dynamic_target = 20;
	let dynamic = Vec::<Particle>::new();
	let stat = vec![Particle{pos: vec2(0.0, 0.0), vel: vec2(0.0, 0.0), r: 0.1}];

	let world_radius = 1.0;

	let visible_distance = 1.0;

    loop {
		clear_background(BLACK);

		let aspect = screen_width() / screen_height();
		let zoom = 1.0/visible_distance;
		let zoom = if aspect >= 1.0 { vec2(zoom/aspect, zoom) } else { vec2(zoom, zoom*aspect) };
		let camera = Camera2D {
			target: vec2(0.0, 0.0),
			zoom,
			..Default::default()
		};
		let pixel = (camera.screen_to_world(vec2(1.0, 0.0)).x - camera.screen_to_world(vec2(0.0, 0.0)).x).abs();
		set_camera(&camera);

		if dynamic.len() < dynamic_target {
			let angle = rand::gen_range(0.0, 2.0*PI);
		}

		draw_circle_lines(0.0, 0.0, world_radius, 2.0*pixel, WHITE);

		next_frame().await;
	}
}
