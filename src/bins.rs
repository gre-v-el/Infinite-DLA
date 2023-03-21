use std::slice::Iter;

use macroquad::prelude::{Vec2, Rect};

use crate::{particle::{StaticParticle, DynamicParticle}, BIN_COUNT, BIN_MARGIN, PARTICLE_R};

pub struct Bins {
	particles: Vec<StaticParticle>, // sorted by bin id
	bins: Vec<usize>, // id of the first particle in nth bin. The last number stores the particle count 
	xmin: f32,
	xmax: f32,
	ymin: f32,
	ymax: f32,
}

impl Bins {
	pub fn new() -> Self {
		let mut bins = Vec::new();
		for _ in 0..(BIN_COUNT*BIN_COUNT + 1) {
			bins.push(0);
		}
		Self { 
			particles: Vec::new(), 
			bins,
			xmin: 0.0,
			xmax: 0.0,
			ymin: 0.0,
			ymax: 0.0,
		}
	}

	// todo: terrible naming
	pub fn get_bin(&self, p: &StaticParticle) -> Option<usize> {
		Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, p.pos)
	}
	pub fn get_bin_dynamic(&self, p: &DynamicParticle) -> Option<usize> {
		Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, p.pos)
	}
	pub fn get_bin_static(xmin: f32, xmax: f32, ymin: f32, ymax: f32, pos: Vec2) -> Option<usize> {
		let x = (pos.x - xmin) / (xmax - xmin) * BIN_COUNT as f32;
		let y = (pos.y - ymin) / (ymax - ymin) * BIN_COUNT as f32;

		if x > 0.0 && x < BIN_COUNT as f32 && y > 0.0 && y < BIN_COUNT as f32 {
			Some(x.floor() as usize + y.floor() as usize * BIN_COUNT)
		}
		else {
			None
		}
	}

	pub fn insert(&mut self, p: StaticParticle) {
		self.xmin = self.xmin.min(p.pos.x - BIN_MARGIN);
		self.xmax = self.xmax.max(p.pos.x + BIN_MARGIN);
		self.ymin = self.ymin.min(p.pos.y - BIN_MARGIN);
		self.ymax = self.ymax.max(p.pos.y + BIN_MARGIN);

		if self.particles.len() == 0 {
			self.xmin = p.pos.x - BIN_MARGIN;
			self.xmax = p.pos.x + BIN_MARGIN;
			self.ymin = p.pos.y - BIN_MARGIN;
			self.ymax = p.pos.y + BIN_MARGIN;
		}
		if let Some(bin) =  self.get_bin(&p) {
			self.particles.insert(self.bins[bin], p);
			for i in (bin+1)..self.bins.len() {
				self.bins[i] += 1;
			}
		}
		else {
			self.particles.push(p);

			self.particles.sort_by(|p1, p2| {
				Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, p2.pos).unwrap()
				.cmp(&Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, p1.pos).unwrap())
			}); // todo: consider cached key, consider implementing the sorting algorithm by hand to modify bins ids on the fly

			let mut bin_done = 0;
			for (i, p) in self.particles.iter().enumerate() {
				let bin = self.get_bin(p).unwrap();
				if bin > bin_done {
					self.bins[bin] = i;
					bin_done = bin;
				}
			}

			for b in bin_done..self.bins.len() {
				self.bins[b] = self.particles.len();
			}
		}
	}

	pub fn get_colliding(&self, p: &DynamicParticle) -> Option<StaticParticle> {
		let bin = self.get_bin_dynamic(p)?;
		// let x = bin%BIN_COUNT;
		// let y = bin/BIN_COUNT;
		
		// for r in -1..=1 {
		// 	let start = (x as i32 - 1).clamp(0, BIN_COUNT as i32 - 1)+(y as i32 + r).clamp(0, BIN_COUNT as i32 - 1)*BIN_COUNT as i32;
		// 	let end   = (x as i32 + 1).clamp(0, BIN_COUNT as i32 - 1)+(y as i32 + r).clamp(0, BIN_COUNT as i32 - 1)*BIN_COUNT as i32;
		// 	// print!("bins.x: {}..{}  ", (x as i32 - 1).clamp(0, BIN_COUNT as i32 - 1), (x as i32 + 1).clamp(0, BIN_COUNT as i32 - 1));
		// 	// print!("bins.y: {}  ", (y as i32 + r).clamp(0, BIN_COUNT as i32 - 1));
		// 	// print!("bins.id: {}..{}  ", start, end);
		// 	// println!("particles.id: {}..{}", self.bins[start as usize], self.bins[end as usize]);
		// 	// println!("{:?}\n", self.bins);
		// 	for other in &self.particles[self.bins[start as usize]..self.bins[end as usize]] {
		// 		if p.pos.distance_squared(other.pos) <= PARTICLE_R * PARTICLE_R && other.pos != p.pos {
		// 			return Some(*other);
		// 		} 
		// 	}
		// }

		for other in &self.particles[self.bins[bin]..self.bins[bin+1]] {
			if p.collides(other) && other.pos != p.pos {
				return Some(*other);
			}
		}
		
		None
	}

	pub fn iter(&self) -> Iter<StaticParticle> {
		self.particles.iter()
	}

	pub fn rect(&self) -> Rect {
		Rect { x: self.xmin, y: self.ymin, w: self.xmax - self.xmin, h: self.ymax - self.ymin }
	}
}