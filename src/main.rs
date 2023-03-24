mod particle;
mod dla;
mod bins;

use std::time::{SystemTime, UNIX_EPOCH};

use dla::DLA;
use egui_macroquad::{egui, macroquad};
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
	let mut draw_particles = false;
	let mut draw_aggregate = false;
	let mut draw_lines = true;
	let mut draw_bins = false;
	let mut draw_world = false;

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
		
		clear_background(BLACK);
		
		let pixel = dla.update_camera();
		if draw_particles 	{ dla.draw_dynamic(); }
		if draw_aggregate 	{ dla.draw_aggregate(); }
		if draw_lines		{ dla.draw_lines(pixel*0.5); }
		if draw_bins		{ dla.draw_bins(pixel*0.5); }
		if draw_world		{ dla.draw_world(); }

		egui_macroquad::ui(|ctx| {
			egui::Window::new("options")
				.collapsible(false)
				.fixed_pos((10.0, 10.0))
				.fixed_size((150.0, 400.0))
				.title_bar(false)
				.show(ctx, |ui| {
					ui.heading("Controls");
					if ui.button("reset").clicked() {
						dla = DLA::new();
					}

					ui.separator();
					ui.heading("Display");
					ui.checkbox(&mut draw_particles, "particles");
					ui.checkbox(&mut draw_aggregate, "aggregate");
					ui.checkbox(&mut draw_lines, "branches");
					ui.checkbox(&mut draw_bins, "bins");
					ui.checkbox(&mut draw_world, "world");

					ui.separator();
					ui.heading("Info");
					ui.label(format!("{}fps", get_fps()));
					ui.label(format!("frame time: {:.2}ms", 1000.0*prof[1]));
					ui.label(format!("update time: {:.2}ms", 1000.0*prof[0]));
					ui.label(format!("draw time: {:.2}ms", 1000.0*prof[1] - 1000.0*prof[0]));
				});
		});
		egui_macroquad::draw();

		profiler[1] = get_time() - start;

		for i in 0..profiler.len() {
			profiler[i] += 10.0*prof[i];
			profiler[i] /= 11.0;
		}

		next_frame().await;
	}
}