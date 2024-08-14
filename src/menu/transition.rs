use bevy::prelude::*;

use super::MenuState;
use crate::AppState;

pub struct MenuTransitionPlugin;

impl Plugin for MenuTransitionPlugin {
	fn build(&self, app: &mut App) {
		app.add_event::<TransitionEvent>().add_systems(
			Update,
			handle_event_transition.run_if(on_event::<TransitionEvent>())
		);
	}
}

#[derive(Clone, Copy, Event)]
pub enum TransitionEvent {
	StartGame,
	ContinueGame,
	Settings,
	Exit
}

pub fn handle_event_transition(
	mut er: EventReader<TransitionEvent>,
	mut ew_app_exit: EventWriter<AppExit>,
	mut menu_state: ResMut<NextState<MenuState>>,
	mut game_state: ResMut<NextState<AppState>>
) {
	let Some(event) = er.read().last() else {
		return;
	};

	use TransitionEvent::*;
	match event {
		StartGame => {
			game_state.set(AppState::Game);
			menu_state.set(MenuState::Disabled);
		},
		ContinueGame => {},
		Settings => {
			menu_state.set(MenuState::Settings);
		},
		Exit => {
			ew_app_exit.send(AppExit::Success);
		}
	}
}
