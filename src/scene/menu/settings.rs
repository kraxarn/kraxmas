impl super::Menu {
	pub fn settings(&mut self, ctx: &egui::CtxRef) {
		let audio_settings = &mut self.audio_settings;
		let window_settings = &mut self.window_settings;

		egui::Window::new("Settings")
			.open(&mut self.settings_open)
			.show(ctx, |ui| {
				ui.heading("Audio");
				egui::Grid::new("audio_grid").show(ui, |ui| {
					ui.label("Music");
					ui.add(egui::Slider::new(&mut audio_settings.music_volume, 0..=100));
					ui.end_row();

					ui.label("Sound");
					ui.add(egui::Slider::new(&mut audio_settings.sound_volume, 0..=100));
					ui.end_row();
				});

				ui.heading("Window");
				egui::Grid::new("window_grid").show(ui, |ui| {
					let resolution = crate::settings::window::base_resolution()
						* window_settings.resolution_scale;
					ui.label("Resolution");
					egui::ComboBox::from_label("")
						.selected_text(format!("{}x{}", resolution.x, resolution.y))
						.show_ui(ui, |ui| {
							for scale in crate::settings::window::all_scales() {
								let res = crate::settings::window::base_resolution() * scale;
								ui.selectable_value(
									&mut window_settings.resolution_scale,
									1_f32,
									format!("{}x{}", res.x, res.y),
								);
							}
						});
					ui.end_row();
				});
			});
	}
}
