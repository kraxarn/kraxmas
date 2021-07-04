use macroquad::prelude::*;

pub struct WindowSettings {
	pub fullscreen: bool,
	pub resolution_scale: f32,
	pub resolution: u8,
}

impl WindowSettings {
	pub fn new() -> Self {
		Self {
			fullscreen: false,
			resolution_scale: 1_f32,
			resolution: 1,
		}
	}
}

pub fn all_scales() -> Vec<f32> {
	vec![0.75_f32, 1_f32, 1.5_f32, 2_f32]
}

pub fn base_resolution() -> glam::Vec2 {
	vec2(1280_f32, 720_f32)
}

pub fn all_resolutions() -> Vec<glam::Vec2> {
	all_scales().into_iter().map(|r| {
		base_resolution() * r
	}).collect()
}
