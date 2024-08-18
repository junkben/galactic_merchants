use bevy::prelude::*;

pub fn window_plugin() -> WindowPlugin {
	WindowPlugin {
		primary_window: Some(Window {
			title: "[TEST] Galactic Merchants".to_string(),
			resolution: (1080.0, 720.0).into(),
			mode: bevy::window::WindowMode::Windowed,
			present_mode: bevy::window::PresentMode::AutoVsync,
			..default()
		}),
		..default()
	}
}
