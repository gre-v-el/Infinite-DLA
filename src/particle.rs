use macroquad::prelude::*;

use crate::{MUTATE_AMOUNT, PARTICLE_R};

#[derive(Clone, Copy)]
pub struct DynamicParticle {
	pub pos: Vec2,
	pub vel: Vec2,
}

impl DynamicParticle {
	pub fn collides(&self, other: &StaticParticle) -> bool {
		(self.pos-other.pos).length_squared() <= 4.0*PARTICLE_R*PARTICLE_R
	}

	// pos - position to normalize the distance to
	pub fn to_static(&self, p: &StaticParticle) -> StaticParticle {
		StaticParticle { pos: p.pos + (self.pos - p.pos).normalize_or_zero()*(2.0 * PARTICLE_R), color: mutate_col(&p.color, MUTATE_AMOUNT)}
	}
}

#[derive(Clone, Copy)]
pub struct StaticParticle {
	pub pos: Vec2,
	pub color: Color,
}



pub fn mutate_col(col: &Color, amount: f32) -> Color {
	Color { 
		r: (col.r + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		g: (col.g + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		b: (col.b + rand::gen_range(-amount, amount)).clamp(0.0, 1.0), 
		a: col.a 
	}
}