use macroquad::prelude::*;

use crate::MUTATE_AMOUNT;

#[derive(Clone, Copy)]
pub struct DynamicParticle {
	pub pos: Vec2,
	pub vel: Vec2,
	pub r: f32,
}

impl DynamicParticle {
	pub fn collides(&self, other: &StaticParticle) -> bool {
		(self.pos-other.pos).length_squared() <= (self.r + other.r)*(self.r + other.r)
	}

	// pos - position to normalize the distance to
	pub fn to_static(&self, p: StaticParticle) -> StaticParticle {
		StaticParticle { pos: p.pos + (self.pos - p.pos).normalize_or_zero()*(p.r + self.r), r: self.r, color: mutate_col(&p.color, MUTATE_AMOUNT)}
	}
}

#[derive(Clone, Copy)]
pub struct StaticParticle {
	pub pos: Vec2,
	pub r: f32,
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