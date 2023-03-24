use std::slice::Iter;
use egui_macroquad::macroquad;
use macroquad::prelude::{Vec2, Rect, vec2};

use crate::{particle::{StaticParticle, DynamicParticle}, BIN_COUNT, BIN_MARGIN_MIN, BIN_MARGIN_MAX, PARTICLE_R};

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
	pub fn get_bin(&self, pos: Vec2) -> Option<usize> {
		Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, pos)
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
		
		// if too close to an edge or outside, resize and rebin
		if p.pos.x < self.xmin + BIN_MARGIN_MIN || p.pos.x > self.xmax - BIN_MARGIN_MIN || p.pos.y < self.ymin + BIN_MARGIN_MIN || p.pos.y > self.ymax - BIN_MARGIN_MIN {

			self.xmin = self.xmin.min(p.pos.x - BIN_MARGIN_MAX);
			self.xmax = self.xmax.max(p.pos.x + BIN_MARGIN_MAX);
			self.ymin = self.ymin.min(p.pos.y - BIN_MARGIN_MAX);
			self.ymax = self.ymax.max(p.pos.y + BIN_MARGIN_MAX);

			self.particles.push(p);

			self.particles.sort_by(|p1, p2| {
				Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, p1.pos).unwrap()
				.cmp(&Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, p2.pos).unwrap())
			}); // todo: consider cached key, consider implementing the sorting algorithm by hand to modify bins ids on the fly

			self.bins[0] = 0;
			let mut bin_done = 0;
			let mut particle_index = 0;
			while particle_index < self.particles.len() {
				let mut do_break = false;
				while self.get_bin(self.particles[particle_index].pos).unwrap() == bin_done {
					particle_index += 1;
					if particle_index >= self.particles.len() {
						do_break = true;
						break;
					}
				}
				if do_break { break; }
				bin_done += 1;
				self.bins[bin_done] = particle_index;
			}

			for b in (bin_done+1)..self.bins.len() {
				self.bins[b] = self.particles.len();
			}
		}
		// if everything's alright, insert
		else {
			let bin = self.get_bin(p.pos).unwrap();

			self.particles.insert(self.bins[bin], p);
			for i in (bin+1)..self.bins.len() {
				self.bins[i] += 1;
			}
		}
	}

	pub fn get_colliding(&self, p: &DynamicParticle) -> Option<StaticParticle> {
		let bin = self.get_bin(p.pos)?;
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
			if p.collides(other) {
				return Some(*other);
			}
		}
		for x in -1..=1 {
			for y in -1..=1 {
				if x == 0 && y == 0 { continue; }
				
				let edge_bin = self.get_bin(p.pos + vec2(x as f32 * PARTICLE_R, y as f32 * PARTICLE_R));
				if let Some(edge_bin) = edge_bin {
					if edge_bin == bin { continue; }
					
					for other in &self.particles[self.bins[edge_bin]..self.bins[edge_bin+1]] {
						if p.collides(other) {
							return Some(*other);
						}
					}
				}
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