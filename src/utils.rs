use macroquad::prelude::*;

pub fn mutate_col(col: &Color, amount: f32) -> Color {
	Color { 
		r: (col.r + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		g: (col.g + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		b: (col.b + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		a: col.a 
	}
}

pub fn update_camera(display_radius: f32) -> f32 {
	let aspect = screen_width() / screen_height();
	let zoom = 1.0/display_radius;
	let zoom = if aspect >= 1.0 { vec2(zoom/aspect, zoom) } else { vec2(zoom, zoom*aspect) };
	let camera = Camera2D {
		target: vec2(0.0, 0.0),
		zoom,
		..Default::default()
	};
	set_camera(&camera);

	(camera.screen_to_world(vec2(0.0, 0.0)).x - camera.screen_to_world(vec2(1.0, 0.0)).x).abs()
}