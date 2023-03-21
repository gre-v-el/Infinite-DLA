mod particle;
mod dla;

use dla::DLA;
use macroquad::prelude::*;

const MUTATE_AMOUNT: f32 = 0.15;
const DYNAMIC_TARGET: usize = 200;
const ZOOM_SMOOTHNESS: f32 = 0.99;
const WORLD_AGGREGATE_RATIO: f32 = 2.0;
const VIEW_AGGREGATE_RATIO: f32 = 1.2;

#[macroquad::main("infinite DLA")]
async fn main() {
	let mut dla = DLA::new();

    loop {
		clear_background(BLACK);

		let pixel = dla.update_camera();

		dla.kinematic_update();
		dla.collide();
		dla.spawn();
		
		dla.draw_lines(pixel);


		next_frame().await;
	}
}
