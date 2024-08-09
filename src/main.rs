use bevy::prelude::*;

mod game;
mod log;
mod window;

fn main() {
	println!("Hello, world!");

	App::new()
		// Declare default plugins plus a custom log and window plugin
		.add_plugins(
			DefaultPlugins
				.set(window::window_plugin())
				.set(log::log_plugin())
		)
		.add_plugins(game::GamePlugin)
		.run();
}
