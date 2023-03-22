mod particle;
mod dla;
mod bins;

use std::time::{SystemTime, UNIX_EPOCH};

use dla::DLA;
use macroquad::{prelude::*, color::hsl_to_rgb};

const MUTATE_AMOUNT: f32 = 0.05;
const DYNAMIC_TARGET: usize = 200;
const ZOOM_SMOOTHNESS: f32 = 0.99;
const WORLD_AGGREGATE_RATIO: f32 = 1.4;
const VIEW_AGGREGATE_RATIO: f32 = 1.2;
const PARTICLE_R: f32 = 0.01;
const GROW_DURATION: f32 = 0.5;

const BIN_COUNT: usize = 15;
const BIN_MARGIN_MIN: f32 = 0.05;
const BIN_MARGIN_MAX: f32 = 0.50;

#[macroquad::main("infinite DLA")]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

	let mut dla = DLA::new();

    loop {
		clear_background(BLACK);

		dla.kinematic_update();
		dla.collide();
		dla.spawn();
		
		let pixel = dla.update_camera();
		// dla.draw_dynamic();
		// dla.draw_aggregate();
		// dla.draw_bins();
		dla.draw_lines(pixel*1.5);



		set_camera(&Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: screen_height() }));
		draw_text_ex(format!("{}fps", get_fps()).as_str(), 10.0, 50.0, TextParams { 
			font_size: 48,
			font_scale: 1.0,
			..Default::default()
		});

		next_frame().await;
	}
}
