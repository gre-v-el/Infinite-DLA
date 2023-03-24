mod particle;
mod dla;
mod bins;

use std::{time::{SystemTime, UNIX_EPOCH}, ops::RangeInclusive};

use dla::DLA;
use egui_macroquad::{egui::{self, epaint::Hsva, Ui, WidgetText}, macroquad};
use macroquad::prelude::*;

fn drag_val_label<N, T>(ui: &mut Ui, val: &mut N, range: RangeInclusive<N>, speed: f64, label: T) where 
	N : egui::emath::Numeric,
	T : Into<WidgetText>
	 {
	ui.horizontal(|ui| {
		ui.add(egui::DragValue::new(val).clamp_range(range).speed(speed));
		ui.label(label);
	});
}

pub struct Globals {
	pub seed_color: Hsva,
	pub branch_thickness: f32,
	pub mutate_amount: f32,
	pub dynamic_target: usize,
	pub zoom_smoothness: f32,
	pub world_aggregate_ratio: f32,
	pub view_aggregate_ratio: f32,
	pub particle_r: f32,
	pub grow_duration: f32,
	pub bin_count: usize,
	pub bin_margin_min: f32,
	pub bin_margin_max: f32,
	pub iters_per_frame: u8,
}

impl Default for Globals {
	fn default() -> Self {
		Globals {
			seed_color: Hsva { h: 0.5, s: 0.0, v: 1.0, a: 1.0 },
			branch_thickness: 2.0,
			mutate_amount: 0.07,
			dynamic_target: 100,
			zoom_smoothness: 0.95,
			world_aggregate_ratio: 1.6,
			view_aggregate_ratio: 1.1,
			particle_r: 0.01,
			grow_duration: 0.3,
			bin_count: 31,
			bin_margin_min: 0.05,
			bin_margin_max: 0.50,
			iters_per_frame: 1,
		}
	}
}

#[macroquad::main("infinite DLA")]
async fn main() {
	rand::srand(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs());

	let mut globals = Globals::default();
	let mut dla = DLA::new(&globals);

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

		for _ in 0..globals.iters_per_frame {
			dla.kinematic_update(&globals);
			dla.collide(&globals);
		}
		dla.spawn(&globals);
		profiler[0] = get_time() - start;
		
		clear_background(BLACK);
		
		let pixel = dla.update_camera(&globals);
		if draw_particles 	{ dla.draw_dynamic(&globals); }
		if draw_aggregate 	{ dla.draw_aggregate(&globals); }
		if draw_lines		{ dla.draw_lines(pixel, &globals); }
		if draw_bins		{ dla.draw_bins(pixel*0.5, &globals); }
		if draw_world		{ dla.draw_world(pixel); }

		egui_macroquad::ui(|ctx| {
			egui::Window::new("options")
				.collapsible(false)
				.fixed_pos((10.0, 10.0))
				.fixed_size((150.0, 400.0))
				.title_bar(false)
				.show(ctx, |ui| {
					ui.heading("Controls");
					if ui.button("restart").clicked() {
						dla = DLA::new(&globals);
					}
					ui.horizontal(|ui| {
						ui.color_edit_button_hsva(&mut globals.seed_color);
						ui.label("Seed color");
					}).response.on_hover_text_at_pointer("Needs restart");
					drag_val_label(ui, &mut globals.mutate_amount, 0.0..=1.0, 0.001, "Color variation");
					drag_val_label(ui, &mut globals.branch_thickness, 0.0..=10.0, 0.01, "Branch thickness");
					drag_val_label(ui, &mut globals.iters_per_frame, 0..=100, 0.25, "Iterations per frame");
					drag_val_label(ui, &mut globals.dynamic_target, 0..=500, 1.0, "Dynamic particles count");
					drag_val_label(ui, &mut globals.zoom_smoothness, 0.5..=0.999, 0.001, "Zoom smoothness");
					drag_val_label(ui, &mut globals.world_aggregate_ratio, 1.1..=3.0, 0.01, "World-aggregate ratio");
					drag_val_label(ui, &mut globals.view_aggregate_ratio, 0.0..=3.0, 0.01, "Zoom-aggregate ratio");
					/*
						world_aggregate_ratio: 1.6,
						view_aggregate_ratio: 1.1,
						particle_r: 0.01,
						grow_duration: 0.3,
						bin_count: 31,
						bin_margin_min: 0.05,
						bin_margin_max: 0.50,
					 */
					if ui.button("defaults").clicked() {
						globals = Globals::default();
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