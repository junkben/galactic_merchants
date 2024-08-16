mod button;
mod main_menu;
mod new_game;
mod transition;

use bevy::prelude::*;

use super::{despawn_screen, AppState, TEXT_COLOR};
use crate::audio::PlayMenuSound;

// This plugin manages the menu, with 4 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub struct MenuPlugin;

impl Plugin for MenuPlugin {
	fn build(&self, app: &mut App) {
		app
			// At start, the menu is not enabled. This will be changed in
			// `menu_setup` when entering the `AppState::Menu` state.
			// Current screen in the menu is handled by an independent state
			// from `AppState`
			.init_state::<MenuState>()
			.add_systems(OnEnter(AppState::Menu), menu_setup)
			// Common systems to all screens that handles buttons behavior
			.add_plugins((
				main_menu::MainMenuPlugin,
				transition::MenuTransitionPlugin,
				button::MenuButtonPlugin,
				new_game::NewGameMenuPlugin
			));
	}
}

// State used for the current menu screen
#[allow(dead_code)]
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
	Main,
	Replay,
	Settings,
	SettingsSound,
	SettingsDisplay,
	ChooseDifficulty,
	#[default]
	Disabled
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
	menu_state.set(MenuState::Main);
}
