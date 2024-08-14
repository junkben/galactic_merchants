use bevy::prelude::*;
use bevy_eventlistener::callbacks::ListenerInput;
use bevy_mod_picking::events::{Click, Pointer};
use button::{MenuButton, MenuButtonEvent, UiMenuButtonExt};
use sickle_ui::prelude::*;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use transition::TransitionEvent;

use super::*;

const MENU_PANEL: Color = Color::srgba(0.15, 0.15, 0.15, 0.50);

// This plugin manages the menu, with 4 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<MainMenuButtonAction>()
			// Systems to handle the main menu screen
			.add_systems(OnEnter(MenuState::Main), main_menu_setup)
			.add_systems(
				OnExit(MenuState::Main),
				despawn_screen::<OnMainMenuScreen>
			)
			.add_systems(
				Update,
				handle_event_do_button_action
					.run_if(on_event::<MainMenuButtonAction>())
			);
	}
}

#[derive(Event)]
pub struct MainMenuButtonAction {
	target: Entity
}

impl From<ListenerInput<Pointer<Click>>> for MainMenuButtonAction {
	fn from(event: ListenerInput<Pointer<Click>>) -> Self {
		MainMenuButtonAction {
			target: event.target
		}
	}
}

impl MenuButtonEvent for MainMenuButtonAction {}

// All actions that can be triggered from a button click
#[derive(Component, Clone, Copy, EnumIter)]
pub enum MainMenuButton {
	StartNewGame,
	LoadSavedGame,
	Settings,
	QuitGame
}

impl<E: MenuButtonEvent> MenuButton<E> for MainMenuButton {
	fn label(&self) -> &'static str {
		use MainMenuButton::*;
		match self {
			StartNewGame => "Start New Game",
			LoadSavedGame => "Load Saved Game",
			Settings => "Settings",
			QuitGame => "Quit Game"
		}
	}
}

// Tag component used to tag entities added on the main menu screen
#[derive(Component)]
struct OnMainMenuScreen;

fn main_menu_setup(mut commands: Commands) {
	commands.ui_builder(UiRoot).column(|column| {
		column
			.style()
			.align_self(AlignSelf::Center)
			.justify_self(JustifySelf::Center)
			.align_items(AlignItems::Center)
			.justify_content(JustifyContent::Center)
			.width(Val::Percent(80.))
			.height(Val::Percent(80.))
			.flex_direction(FlexDirection::Column)
			.background_color(MENU_PANEL);

		column.spawn(OnMainMenuScreen);

		column.column(|button_column| {
			button_column
				.style()
				.align_items(AlignItems::Center)
				.justify_content(JustifyContent::Center)
				.width(Val::Percent(60.))
				.height(Val::Percent(60.));

			button_column
				.label(LabelConfig {
					label: "GALACTIC MERCHANTS".into(),
					color: TEXT_COLOR,
					..default()
				})
				.style()
				.align_self(AlignSelf::Auto)
				.justify_self(JustifySelf::Auto)
				.font_size(80.0)
				.margin(UiRect::all(Val::Px(20.0)));

			// Spawn main menu buttons
			for button in MainMenuButton::iter() {
				button_column.menu_button::<MainMenuButtonAction>(button);
			}
		});
	});
}

fn handle_event_do_button_action(
	mut er: EventReader<MainMenuButtonAction>,
	query_button: Query<(Entity, &MainMenuButton)>,
	mut ew_transition: EventWriter<TransitionEvent>
) {
	let Some(event) = er.read().last() else {
		return;
	};

	let Some((_, action)) =
		query_button.iter().find(|&(e, _)| e == event.target)
	else {
		return;
	};

	use MainMenuButton::*;
	ew_transition.send(match action {
		StartNewGame => TransitionEvent::StartGame,
		LoadSavedGame => TransitionEvent::ContinueGame,
		Settings => TransitionEvent::Settings,
		QuitGame => TransitionEvent::Exit
	});
}
