mod particle;
mod dla;
mod bins;

use std::time::{SystemTime, UNIX_EPOCH};

use dla::DLA;
use macroquad::prelude::*;

const MUTATE_AMOUNT: f32 = 0.07;
const DYNAMIC_TARGET: usize = 100;
const ZOOM_SMOOTHNESS: f32 = 0.95;
const WORLD_AGGREGATE_RATIO: f32 = 1.6;
const VIEW_AGGREGATE_RATIO: f32 = 1.05;
const PARTICLE_R: f32 = 0.01;
const GROW_DURATION: f32 = 0.3;

const BIN_COUNT: usize = 31;
const BIN_MARGIN_MIN: f32 = 0.05;
const BIN_MARGIN_MAX: f32 = 0.50;

const ITERS_PER_FRAME: u8 = 30;

#[macroquad::main("infinite DLA")]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

	let mut dla = DLA::new();
	let mut do_draw = true;

	// time of completion - time of frame start
	// kinematics, collisions, spawning, drawing
	let mut profiler = [0.0; 2];

    loop {
		let start = get_time();
		
		let prof = profiler.clone();

		for _ in 0..ITERS_PER_FRAME {
			dla.kinematic_update();
			dla.collide();
		}
		dla.spawn();
		profiler[0] = get_time() - start;
		
		if is_key_pressed(KeyCode::Space) {
			do_draw = !do_draw;
		}
		
		clear_background(BLACK);
		
		if do_draw {
			let pixel = dla.update_camera();
			// dla.draw_dynamic();
			// dla.draw_aggregate();
			// dla.draw_bins();
			// dla.draw_world();
			dla.draw_lines(pixel*1.5);
		}
		set_camera(&Camera2D::from_display_rect(Rect { x: 0.0, y: 0.0, w: screen_width(), h: screen_height() }));
		draw_text_ex(format!("{}fps", get_fps()).as_str(), 10.0, 50.0, TextParams { 
			font_size: 48,
			font_scale: 1.0,
			..Default::default()
		});
		
		let total = prof[1];
		let kin = (prof[0] / total) as f32;
		let drw = ((prof[1] - prof[0]) / total) as f32;
		draw_rectangle(10.0, screen_height()-60.0, 200.0*kin, 50.0, RED);
		draw_rectangle(10.0+200.0*kin, screen_height()-60.0, 200.0*drw, 50.0, GREEN);

		draw_text_ex(format!("{:.2}ms", total*1000.0).as_str(), 210.0, screen_height()-30.0, TextParams { font_size: 36, ..Default::default()});

		profiler[1] = get_time() - start;

		for i in 0..profiler.len() {
			profiler[i] += 10.0*prof[i];
			profiler[i] /= 11.0;
		}

		next_frame().await;
	}
}
