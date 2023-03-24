use std::f32::consts::PI;
use egui_macroquad::macroquad;
use macroquad::prelude::*;

use crate::{particle::{DynamicParticle, StaticParticle}, bins::Bins, Globals};

pub struct DLA {
	dynamic: Vec<DynamicParticle>,
	bins: Bins,
	lines: Vec<(Vec2, Vec2, Color, f32)>,
	world_radius: f32,
	display_radius_target: f32,
	display_radius: f32,
}

impl DLA {
	pub fn new(globals: &Globals) -> Self {
		let mut bins = Bins::new(globals);
		bins.insert(StaticParticle { pos: vec2(0.0, 0.0), color: globals.seed_color }, globals);

		DLA { 
			dynamic: Vec::<DynamicParticle>::new(), 
			bins,
			lines: Vec::new(), 
			world_radius: 1.0, 
			display_radius_target: 0.1, 
			display_radius: 0.08, 
		}
	}

	pub fn update_camera(&mut self, globals: &Globals) -> f32 {
		self.display_radius = self.display_radius * globals.zoom_smoothness + self.display_radius_target * (1.0 - globals.zoom_smoothness);

		let aspect = screen_width() / screen_height();
		let zoom = 1.0/self.display_radius;
		let zoom = if aspect >= 1.0 { vec2(zoom/aspect, zoom) } else { vec2(zoom, zoom*aspect) };
		let camera = Camera2D {
			target: vec2(0.0, 0.0),
			zoom,
			..Default::default()
		};
		set_camera(&camera);
	
		(camera.screen_to_world(vec2(0.0, 0.0)).x - camera.screen_to_world(vec2(1.0, 0.0)).x).abs()
	}

	pub fn kinematic_update(&mut self, globals: &Globals) {
		for p in &mut self.dynamic {
			p.pos += p.vel * globals.particle_r;

			if p.pos.length_squared() >= self.world_radius*self.world_radius {
				let pos_norm = p.pos.normalize_or_zero();
				p.pos = pos_norm*self.world_radius;
				let normal = -pos_norm;
				p.vel = p.vel - 2.0 * (p.vel.dot(normal)) * normal;

				let rotator = vec2(1.0, rand::gen_range(-0.5, 0.5)).normalize();
				p.vel = rotator.rotate(p.vel);
			}
		}
	}

	pub fn collide(&mut self, globals: &Globals) {
		self.dynamic.retain(|p| {
			let collided = self.bins.get_colliding(p, globals);
			// let mut collided = None;
			// for s in self.bins.iter() {
			// 	if p.collides(&s) {
			// 		collided = Some(*s);
			// 		break;
			// 	}
			// }

			if let Some(agg) = collided {
				let new = p.to_static(&agg, globals);
				self.bins.insert(new, globals);
				// hsl_to_rgb((get_time() as f32 * 0.05)%1.0, 1.0, 0.5)
				// hsl_to_rgb(new.pos.length()*2.0 % 1.0, 1.0, 0.5)
				self.lines.push((agg.pos, new.pos, new.color, get_time() as f32));
				self.world_radius = self.world_radius.max(p.pos.length()*globals.world_aggregate_ratio);
				self.display_radius_target = self.display_radius_target.max(p.pos.length()*globals.view_aggregate_ratio);
			}

			return collided.is_none();
		});
	}

	pub fn spawn(&mut self, globals: &Globals) {
		while self.dynamic.len() < globals.dynamic_target {
			let spread = 0.8;

			let pos_angle = rand::gen_range(0.0, 2.0*PI);
			let vel_angle = rand::gen_range(pos_angle + PI - spread, pos_angle + PI + spread);

			self.dynamic.push(
				DynamicParticle { 
					pos: vec2(self.world_radius*pos_angle.cos(), self.world_radius*pos_angle.sin()), 
					vel: vec2(vel_angle.cos(), vel_angle.sin()),
				}
			);
		}
	}

	pub fn draw_aggregate(&self, globals: &Globals) {
		for particle in self.bins.iter() {
			// let mut col = 0;
			// for (bin_index, starting_particle) in self.bins.bins.iter().rev().enumerate() {
			// 	if particle_index >= *starting_particle {
			// 		col = bin_index;
			// 		break;
			// 	}
			// }
			// let col = color_u8!((col%4) * 80, (col%25)*10, (col&2) * 120, 255);
			draw_circle(particle.pos.x, particle.pos.y, globals.particle_r, particle.color);
		}
	}

	pub fn draw_lines(&self, thickness: f32, globals: &Globals) {
		for (start, end, col, spawn) in &self.lines {
			let t = ((get_time() as f32 - spawn) / globals.grow_duration).clamp(0.0, 1.0);
			let t = 3.0*t*t - 2.0*t*t*t;
			draw_line(start.x, start.y, end.x * t + start.x * (1.0-t), end.y * t + start.y * (1.0 - t), thickness*globals.branch_thickness, *col);
		}
	}

	pub fn draw_dynamic(&self, globals: &Globals) {
		for p in &self.dynamic {
			draw_circle(p.pos.x, p.pos.y, globals.particle_r, DARKBROWN);
		}
	}

	pub fn draw_bins(&self, pixel: f32, globals: &Globals) {
		let rect = self.bins.rect();
		for i in 0..=globals.bin_count {
			draw_line(rect.left() + rect.w * i as f32 / globals.bin_count as f32, rect.top(), rect.left() + rect.w * i as f32 / globals.bin_count as f32, rect.bottom(), pixel, GRAY);
			draw_line(rect.left(), rect.top() + rect.h * i as f32 / globals.bin_count as f32, rect.right(), rect.top() + rect.h * i as f32 / globals.bin_count as f32, pixel, GRAY);
		}
	}

	pub fn draw_world(&self, pixel: f32) {
		draw_circle_lines(0.0, 0.0, self.world_radius, pixel, GRAY);
	}
}