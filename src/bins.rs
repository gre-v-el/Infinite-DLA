use std::slice::Iter;
use egui_macroquad::macroquad;
use macroquad::prelude::{Vec2, Rect, vec2};

use crate::{particle::{StaticParticle, DynamicParticle}, Globals};

pub struct Bins {
	particles: Vec<StaticParticle>, // sorted by bin id
	bins: Vec<usize>, // id of the first particle in nth bin. The last number stores the particle count 
	xmin: f32,
	xmax: f32,
	ymin: f32,
	ymax: f32,
}

impl Bins {
	pub fn new(globals: &Globals) -> Self {
		let mut bins = Vec::new();
		for _ in 0..(globals.bin_count*globals.bin_count + 1) {
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
	pub fn get_bin(&self, pos: Vec2, globals: &Globals) -> Option<usize> {
		Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, pos, globals)
	}

	pub fn get_bin_static(xmin: f32, xmax: f32, ymin: f32, ymax: f32, pos: Vec2, globals: &Globals) -> Option<usize> {
		let x = (pos.x - xmin) / (xmax - xmin) * globals.bin_count as f32;
		let y = (pos.y - ymin) / (ymax - ymin) * globals.bin_count as f32;

		if x > 0.0 && x < globals.bin_count as f32 && y > 0.0 && y < globals.bin_count as f32 {
			Some(x.floor() as usize + y.floor() as usize * globals.bin_count)
		}
		else {
			None
		}
	}

	pub fn insert(&mut self, p: StaticParticle, globals: &Globals) {
		
		// if too close to an edge or outside, resize and rebin
		if p.pos.x < self.xmin + globals.bin_margin_min || p.pos.x > self.xmax - globals.bin_margin_min || p.pos.y < self.ymin + globals.bin_margin_min || p.pos.y > self.ymax - globals.bin_margin_min {

			self.xmin = self.xmin.min(p.pos.x - globals.bin_margin_max);
			self.xmax = self.xmax.max(p.pos.x + globals.bin_margin_max);
			self.ymin = self.ymin.min(p.pos.y - globals.bin_margin_max);
			self.ymax = self.ymax.max(p.pos.y + globals.bin_margin_max);

			self.particles.push(p);

			self.particles.sort_by(|p1, p2| {
				Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, p1.pos, globals).unwrap()
				.cmp(&Self::get_bin_static(self.xmin, self.xmax, self.ymin, self.ymax, p2.pos, globals).unwrap())
			}); // todo: consider cached key, consider implementing the sorting algorithm by hand to modify bins ids on the fly

			self.bins[0] = 0;
			let mut bin_done = 0;
			let mut particle_index = 0;
			while particle_index < self.particles.len() {
				let mut do_break = false;
				while self.get_bin(self.particles[particle_index].pos, globals).unwrap() == bin_done {
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
			let bin = self.get_bin(p.pos, globals).unwrap();

			self.particles.insert(self.bins[bin], p);
			for i in (bin+1)..self.bins.len() {
				self.bins[i] += 1;
			}
		}
	}

	pub fn get_colliding(&self, p: &DynamicParticle, globals: &Globals) -> Option<StaticParticle> {
		let bin = self.get_bin(p.pos, globals)?;

		for other in &self.particles[self.bins[bin]..self.bins[bin+1]] {
			if p.collides(other, globals) {
				return Some(*other);
			}
		}
		for x in -1..=1 {
			for y in -1..=1 {
				if x == 0 && y == 0 { continue; }
				
				let edge_bin = self.get_bin(p.pos + vec2(x as f32 * globals.particle_r, y as f32 * globals.particle_r), globals);
				if let Some(edge_bin) = edge_bin {
					if edge_bin == bin { continue; }
					
					for other in &self.particles[self.bins[edge_bin]..self.bins[edge_bin+1]] {
						if p.collides(other, globals) {
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