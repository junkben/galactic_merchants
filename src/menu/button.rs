use bevy::prelude::*;
use bevy_eventlistener::{callbacks::ListenerInput, event_listener::On};
use bevy_mod_picking::events::{Click, Pointer};
use sickle_ui::{prelude::*, ui_commands::SetTextExt};

use super::*;

const NORMAL_BUTTON: Color = Color::srgba(0.15, 0.15, 0.15, 0.50);
const HOVERED_BUTTON: Color = Color::srgba(0.25, 0.25, 0.25, 0.50);
const HOVERED_PRESSED_BUTTON: Color = Color::srgba(0.25, 0.65, 0.25, 0.50);
const PRESSED_BUTTON: Color = Color::srgba(0.35, 0.75, 0.35, 0.50);

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

pub struct MenuButtonPlugin;

impl Plugin for MenuButtonPlugin {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, button_system.run_if(in_state(AppState::Menu)));
	}
}

pub trait MenuButtonEvent: Event + From<ListenerInput<Pointer<Click>>> {}

pub trait MenuButton<E>
where
	Self: Component + Clone,
	E: MenuButtonEvent
{
	fn label(&self) -> &'static str;

	/// Common style for all buttons on the main menu
	fn bundle(&self) -> ButtonBundle {
		ButtonBundle {
			style: Style {
				width: Val::Percent(80.),
				height: Val::Percent(25.),
				min_width: Val::Px(300.),
				min_height: Val::Px(30.),
				margin: UiRect::all(Val::Px(5.)),
				align_self: AlignSelf::Auto,
				justify_self: JustifySelf::Auto,
				justify_content: JustifyContent::Center,
				align_items: AlignItems::Center,
				..default()
			},
			background_color: NORMAL_BUTTON.into(),
			..default()
		}
	}
}

pub trait UiMenuButtonExt<'w, 's> {
	fn menu_button<'a, E>(
		&'a mut self,
		button: impl MenuButton<E>
	) -> UiBuilder<'a, Entity>
	where
		E: MenuButtonEvent;
}

impl<'w, 's> UiMenuButtonExt<'w, 's> for UiBuilder<'_, Entity> {
	fn menu_button<'a, E>(
		&'a mut self,
		button: impl MenuButton<E>
	) -> UiBuilder<'a, Entity>
	where
		E: MenuButtonEvent
	{
		self.container(
			(
				button.bundle(),
				button.clone(),
				On::<Pointer<Click>>::send_event::<E>()
			),
			|builder| {
				builder
					.label(LabelConfig::default())
					.style()
					.font_size(40.0)
					.font_color(TEXT_COLOR)
					.entity_commands()
					.insert(button.clone())
					.set_text(button.label(), None);
			}
		)
	}
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
	mut query_button: Query<
		(&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
		(Changed<Interaction>, With<Button>)
	>,
	mut ew_menu_sound: EventWriter<PlayMenuSound>
) {
	for (interaction, mut color, selected) in &mut query_button {
		match (*interaction, selected) {
			(Interaction::Pressed, _) | (Interaction::None, Some(_)) => {
				*color = PRESSED_BUTTON.into();
			},
			(Interaction::Hovered, Some(_)) => {
				*color = HOVERED_PRESSED_BUTTON.into();
				ew_menu_sound.send(PlayMenuSound::ButtonPressed);
			},
			(Interaction::Hovered, None) => {
				*color = HOVERED_BUTTON.into();
				ew_menu_sound.send(PlayMenuSound::ButtonHovered);
			},
			(Interaction::None, None) => {
				*color = NORMAL_BUTTON.into();
			}
		}
	}
}
