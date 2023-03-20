use std::f32::consts::PI;

use macroquad::prelude::*;

const MUTATE_AMOUNT: f32 = 0.1;

fn mutate_col(col: &Color, amount: f32) -> Color {
	Color { 
		r: (col.r + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		g: (col.g + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		b: (col.b + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		a: col.a 
	}
}

struct DynamicParticle {
	pos: Vec2,
	vel: Vec2,
	r: f32,
}

impl DynamicParticle {
	fn collides(&self, other: &StaticParticle) -> bool {
		(self.pos-other.pos).length_squared() <= (self.r + other.r)*(self.r + other.r)
	}

	// pos - position to normalize the distance to
	fn to_static(&self, pos: Vec2, r: f32, col: Color) -> StaticParticle {
		StaticParticle { pos: pos + (self.pos - pos).normalize_or_zero()*(r + self.r), r: self.r, color: mutate_col(&col, MUTATE_AMOUNT)}
	}
}

struct StaticParticle {
	pos: Vec2,
	r: f32,
	color: Color,
}


#[macroquad::main("infinite DLA")]
async fn main() {
	let dynamic_target = 200;
	let mut dynamic = Vec::<DynamicParticle>::new();
	let mut aggregate = vec![StaticParticle{pos: vec2(0.0, 0.0), r: 0.02, color: WHITE}];

	let mut world_radius = 1.0;
	let mut display_radius_target: f32 = 0.1;
	let mut display_radius = 0.1;

    loop {
		clear_background(BLACK);

		let aspect = screen_width() / screen_height();
		let zoom = 1.0/display_radius;
		let zoom = if aspect >= 1.0 { vec2(zoom/aspect, zoom) } else { vec2(zoom, zoom*aspect) };
		let camera = Camera2D {
			target: vec2(0.0, 0.0),
			zoom,
			..Default::default()
		};
		let pixel = (camera.screen_to_world(vec2(1.0, 0.0)).x - camera.screen_to_world(vec2(0.0, 0.0)).x).abs();
		set_camera(&camera);

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
					collided = Some((s.pos, s.r, s.color));
					break;
				}
			}

			if let Some((pos, r, col)) = collided {
				aggregate.push(p.to_static(pos, r, col));
				world_radius = world_radius.max(p.pos.length()*3.0);
				display_radius_target = display_radius_target.max(p.pos.length()*1.2);
			}

			return collided.is_none();
		});

		if dynamic.len() < dynamic_target {
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

		draw_circle_lines(0.0, 0.0, world_radius, 2.0*pixel, WHITE);

		for p in &aggregate {
			draw_circle(p.pos.x, p.pos.y, p.r, p.color);
		}
		for p in &dynamic {
			draw_circle(p.pos.x, p.pos.y, p.r, DARKBROWN);
		}

		display_radius = display_radius * 0.99 + display_radius_target * 0.01;

		next_frame().await;
	}
}
