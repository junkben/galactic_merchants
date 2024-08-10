use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use menu::MenuState;

mod audio;
mod camera;
mod game;
mod log;
mod menu;
mod splash;
mod window;

const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

fn main() {
	println!("Hello, world!");
	let mut app = App::new();

	// Declare default plugins plus a custom log and window plugin
	app.add_plugins(
		DefaultPlugins
			.set(window::window_plugin())
			.set(log::log_plugin())
	);

	// Add game state
	app.init_state::<AppState>();

	// Adds the plugins for each state
	app.add_plugins((
		DefaultPickingPlugins,
		camera::CameraPlugin,
		splash::SplashPlugin,
		menu::MenuPlugin,
		game::GamePlugin,
		audio::MenuAudioPlugin
	));

	// Add update handles
	app.add_systems(
		Update,
		(
			transition_to_splash.run_if(in_state(AppState::Setup)),
			handle_user_open_menu.run_if(in_state(AppState::Game)),
			handle_user_close_menu.run_if(in_state(AppState::Menu))
		)
	);

	// Run the app
	match app.run() {
		AppExit::Success => info!("Successfully exited."),
		AppExit::Error(err) => panic!("Error code [{err}].")
	}
}

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
	#[default]
	Setup,
	Splash,
	Menu,
	Game
}

fn transition_to_splash(mut app_state: ResMut<NextState<AppState>>) {
	app_state.set(AppState::Splash);
}

// Generic system that takes a component as a parameter, and will despawn all
// entities with that component
fn despawn_screen<T: Component>(
	query_entities_to_despawn: Query<Entity, With<T>>,
	mut commands: Commands
) {
	for entity in &query_entities_to_despawn {
		commands.entity(entity).despawn_recursive();
	}
}

fn handle_user_open_menu(
	mut game_state: ResMut<NextState<AppState>>,
	mut menu_state: ResMut<NextState<MenuState>>,
	keys: Res<ButtonInput<KeyCode>>
) {
	if keys.just_pressed(KeyCode::Escape) {
		debug!("moving to menu from game");
		game_state.set(AppState::Menu);
		menu_state.set(MenuState::Main);
	}
}

fn handle_user_close_menu(
	mut game_state: ResMut<NextState<AppState>>,
	mut menu_state: ResMut<NextState<MenuState>>,
	keys: Res<ButtonInput<KeyCode>>
) {
	if keys.just_pressed(KeyCode::Escape) {
		debug!("moving to game from menu");
		game_state.set(AppState::Game);
		menu_state.set(MenuState::Disabled);
	}
}
