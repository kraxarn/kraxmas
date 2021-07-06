use crate::scene::Scene;
use macroquad::prelude::*;

mod color;
mod menu;
mod scene;

pub const APP_NAME: &str = "OpenTD: Alpha";

fn window_conf() -> Conf {
	Conf {
		window_title: APP_NAME.to_owned(),
		window_width: 1280_i32,
		window_height: 720_i32,
		..Default::default()
	}
}

#[macroquad::main(window_conf)]
async fn main() {
	let mut menu = menu::Menu::new();

	loop {
		clear_background(color::BACKGROUND);

		menu.update();

		draw_text(
			&format!("FPS: {}", get_fps()),
			16_f32,
			24_f32,
			22_f32,
			color::FOREGROUND,
		);

		next_frame().await
	}
}
