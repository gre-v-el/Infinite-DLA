mod particle;
mod dla;

use std::time::{SystemTime, UNIX_EPOCH};

use dla::DLA;
use macroquad::prelude::*;

const MUTATE_AMOUNT: f32 = 0.15;
const DYNAMIC_TARGET: usize = 200;
const ZOOM_SMOOTHNESS: f32 = 0.99;
const WORLD_AGGREGATE_RATIO: f32 = 1.4;
const VIEW_AGGREGATE_RATIO: f32 = 1.2;
const PARTICLE_R: f32 = 0.01;
const GROW_DURATION: f32 = 0.5;

#[macroquad::main("infinite DLA")]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

	let mut dla = DLA::new();

    loop {
		clear_background(BLACK);

		let pixel = dla.update_camera();

		dla.kinematic_update();
		dla.collide();
		dla.spawn();
		
		dla.draw_lines(pixel*1.5);


		next_frame().await;
	}
}
