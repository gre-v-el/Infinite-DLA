use std::f32::consts::PI;

use macroquad::prelude::*;

use crate::{particle::{DynamicParticle, StaticParticle}, WORLD_AGGREGATE_RATIO, VIEW_AGGREGATE_RATIO, DYNAMIC_TARGET, ZOOM_SMOOTHNESS, PARTICLE_R, GROW_DURATION};

pub struct DLA {
	dynamic: Vec<DynamicParticle>,
	aggregate: Vec<StaticParticle>,
	lines: Vec<(Vec2, Vec2, Color, f32)>,
	world_radius: f32,
	display_radius_target: f32,
	display_radius: f32,
}

impl DLA {
	pub fn new() -> Self {
		DLA { 
			dynamic: Vec::<DynamicParticle>::new(), 
			aggregate: vec![StaticParticle{pos: vec2(0.0, 0.0), color: WHITE}], 
			lines: Vec::new(), 
			world_radius: 1.0, 
			display_radius_target: 0.1, 
			display_radius: 0.08, 
		}
	}

	pub fn update_camera(&mut self) -> f32 {
		self.display_radius = self.display_radius * ZOOM_SMOOTHNESS + self.display_radius_target * (1.0 - ZOOM_SMOOTHNESS);

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

	pub fn kinematic_update(&mut self) {
		for p in &mut self.dynamic {
			p.pos += p.vel * 0.01;

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

	pub fn collide(&mut self) {
		self.dynamic.retain(|p| {
			let mut collided = None;
			for s in &self.aggregate {
				if p.collides(&s) {
					collided = Some(*s);
					break;
				}
			}

			if let Some(agg) = collided {
				let new = p.to_static(agg);
				self.aggregate.push(new);
				self.lines.push((agg.pos, new.pos, new.color, get_time() as f32));
				self.world_radius = self.world_radius.max(p.pos.length()*WORLD_AGGREGATE_RATIO);
				self.display_radius_target = self.display_radius_target.max(p.pos.length()*VIEW_AGGREGATE_RATIO);
			}

			return collided.is_none();
		});
	}

	pub fn spawn(&mut self) {
		if self.dynamic.len() < DYNAMIC_TARGET {
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

	pub fn draw_aggregate(&self) {
		for p in &self.aggregate {
			draw_circle(p.pos.x, p.pos.y, PARTICLE_R * 1.2, p.color);
		}
	}

	pub fn draw_lines(&self, thickness: f32) {
		for (start, end, col, spawn) in &self.lines {
			let t = ((get_time() as f32 - spawn) / GROW_DURATION).clamp(0.0, 1.0);
			let t = 3.0*t*t - 2.0*t*t*t;
			draw_line(start.x, start.y, end.x * t + start.x * (1.0-t), end.y * t + start.y * (1.0 - t), thickness*2.0, *col);
		}
	}

	pub fn draw_dynamic(&self) {
		for p in &self.dynamic {
			draw_circle(p.pos.x, p.pos.y, PARTICLE_R, DARKBROWN);
		}
	}
}