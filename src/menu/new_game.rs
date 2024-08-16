use bevy::prelude::*;
use bevy_mod_picking::{
	events::{Click, Pointer},
	prelude::ListenerInput
};
use sickle_ui::prelude::*;
use strum::IntoEnumIterator as _;
use strum_macros::EnumIter;

use super::{
	button::{MenuButton, MenuButtonEvent, UiMenuButtonExt},
	MenuState
};
use crate::{despawn_screen, resources::Difficulty};

pub struct NewGameMenuPlugin;

impl Plugin for NewGameMenuPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<DifficultyMenuAction>()
			// Systems to handle the main menu screen
			.add_systems(
				OnEnter(MenuState::ChooseDifficulty),
				difficulty_menu_setup
			)
			.add_systems(
				OnExit(MenuState::ChooseDifficulty),
				despawn_screen::<OnDifficultyMenuScreen>
			)
			.add_systems(
				Update,
				handle_event_difficulty_menu_action
					.run_if(on_event::<DifficultyMenuAction>())
			);
	}
}

// Tag component used to tag entities added on the difficulty menu screen
#[derive(Component)]
struct OnDifficultyMenuScreen;

#[derive(Event)]
pub struct DifficultyMenuAction {
	target: Entity
}

impl From<ListenerInput<Pointer<Click>>> for DifficultyMenuAction {
	fn from(event: ListenerInput<Pointer<Click>>) -> Self {
		DifficultyMenuAction {
			target: event.target
		}
	}
}

impl MenuButtonEvent for DifficultyMenuAction {}

// All actions that can be triggered from a button click
#[derive(Component, Clone, Copy, EnumIter)]
pub enum DifficultyMenuButton {
	Tutorial,
	Novice,
	Beginner,
	Intermediate,
	Expert,
	Master,
	GoBack
}

impl<E: MenuButtonEvent> MenuButton<E> for DifficultyMenuButton {
	fn label(&self) -> &'static str {
		use DifficultyMenuButton::*;
		match self {
			Tutorial => "Tutorial",
			Novice => "Novice",
			Beginner => "Beginner",
			Intermediate => "Intermediate",
			Expert => "Expert",
			Master => "Master",
			GoBack => "GO BACK"
		}
	}
}

fn difficulty_menu_setup(mut commands: Commands) {
	commands
		.ui_builder(UiRoot)
		.column(|column| {
			column
				.style()
				.align_self(AlignSelf::Center)
				.justify_self(JustifySelf::Center)
				.align_items(AlignItems::Center)
				.justify_content(JustifyContent::Center)
				.width(Val::Percent(80.))
				.height(Val::Percent(80.))
				.flex_direction(FlexDirection::Column)
				.background_color(super::main_menu::MENU_PANEL);

			column.column(|button_column| {
				button_column
					.style()
					.align_items(AlignItems::Center)
					.justify_content(JustifyContent::Center)
					.width(Val::Percent(60.))
					.height(Val::Percent(60.));

				// Spawn main menu buttons
				for button in DifficultyMenuButton::iter() {
					button_column.menu_button::<DifficultyMenuAction>(button);
				}
			});
		})
		.insert(OnDifficultyMenuScreen);
}

fn handle_event_difficulty_menu_action(
	mut commands: Commands,
	mut er: EventReader<DifficultyMenuAction>,
	query_button: Query<(Entity, &DifficultyMenuButton)>,
	mut menu_state: ResMut<NextState<MenuState>>
) {
	let Some(event) = er.read().last() else {
		return;
	};

	let Some((_, action)) =
		query_button.iter().find(|&(e, _)| e == event.target)
	else {
		return;
	};

	use DifficultyMenuButton::*;
	match action {
		Tutorial => {
			commands.insert_resource(Difficulty::Tutorial);
		},
		Novice => {
			commands.insert_resource(Difficulty::Novice);
		},
		Beginner => {
			commands.insert_resource(Difficulty::Beginner);
		},
		Intermediate => {
			commands.insert_resource(Difficulty::Intermediate);
		},
		Expert => {
			commands.insert_resource(Difficulty::Expert);
		},
		Master => {
			commands.insert_resource(Difficulty::Master);
		},
		GoBack => {
			menu_state.set(MenuState::Main);
		}
	};
}
