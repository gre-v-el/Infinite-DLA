mod particle;
mod utils;

use std::f32::consts::PI;

use macroquad::prelude::*;
use particle::{DynamicParticle, StaticParticle};
use utils::update_camera;

const MUTATE_AMOUNT: f32 = 0.05;
const DYNAMIC_TARGET: usize = 200;
const ZOOM_SMOOTHNESS: f32 = 0.99;
const WORLD_AGGREGATE_RATIO: f32 = 2.0;
const VIEW_AGGREGATE_RATIO: f32 = 1.2;

#[macroquad::main("infinite DLA")]
async fn main() {
	let mut dynamic = Vec::<DynamicParticle>::new();
	let mut aggregate = vec![StaticParticle{pos: vec2(0.0, 0.0), r: 0.02, color: WHITE}];
	let mut lines = Vec::new();

	let mut world_radius = 1.0;
	let mut display_radius_target: f32 = 0.1;
	let mut display_radius = 0.09;

    loop {
		clear_background(BLACK);

		let pixel = update_camera(display_radius);

		for p in &mut dynamic {
			p.pos += p.vel * 0.01;

			if p.pos.length_squared() >= world_radius*world_radius {
				let pos_norm = p.pos.normalize_or_zero();
				p.pos = pos_norm*world_radius;
				let normal = -pos_norm;
				p.vel = p.vel - 2.0 * (p.vel.dot(normal)) * normal;

				let rotator = vec2(1.0, rand::gen_range(-0.5, 0.5)).normalize();
				p.vel = rotator.rotate(p.vel);
			}
		}

		dynamic.retain(|p| {
			let mut collided = None;
			for s in &aggregate {
				if p.collides(&s) {
					collided = Some(*s);
					break;
				}
			}

			if let Some(agg) = collided {
				let new = p.to_static(agg);
				aggregate.push(new);
				lines.push((agg.pos, new.pos, new.color));
				world_radius = world_radius.max(p.pos.length()*WORLD_AGGREGATE_RATIO);
				display_radius_target = display_radius_target.max(p.pos.length()*VIEW_AGGREGATE_RATIO);
			}

			return collided.is_none();
		});

		if dynamic.len() < DYNAMIC_TARGET {
			let spread = 0.8;

			let pos_angle = rand::gen_range(0.0, 2.0*PI);
			let vel_angle = rand::gen_range(pos_angle + PI - spread, pos_angle + PI + spread);

			dynamic.push(
				DynamicParticle { 
					pos: vec2(world_radius*pos_angle.cos(), world_radius*pos_angle.sin()), 
					vel: vec2(vel_angle.cos(), vel_angle.sin()), 
					r: 0.01 
				}
			);
		}


		// for p in &aggregate {
		// 	draw_circle(p.pos.x, p.pos.y, p.r*1.2, p.color);
		// }
		for l in &lines {
			draw_line(l.0.x, l.0.y, l.1.x, l.1.y, pixel*2.0, l.2);
		}
		for p in &dynamic {
			draw_circle(p.pos.x, p.pos.y, p.r, DARKBROWN);
		}

		display_radius = display_radius * ZOOM_SMOOTHNESS + display_radius_target * (1.0 - ZOOM_SMOOTHNESS);

		next_frame().await;
	}
}
