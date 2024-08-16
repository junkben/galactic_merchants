use bevy::{
	ecs::system::{EntityCommand, EntityCommands},
	prelude::*
};

pub trait FontCommands<'a> {
	fn set_font(
		&'a mut self,
		path: impl Into<String>
	) -> &mut EntityCommands<'a>;
	fn set_font_size(&'a mut self, size: f32) -> &mut EntityCommands<'a>;
	fn set_font_color(&'a mut self, color: Color) -> &mut EntityCommands<'a>;
}

impl<'a> FontCommands<'a> for EntityCommands<'a> {
	fn set_font(
		&'a mut self,
		path: impl Into<String>
	) -> &mut EntityCommands<'a> {
		self.add(SetFont(path.into()))
	}

	fn set_font_size(&'a mut self, size: f32) -> &mut EntityCommands<'a> {
		self.add(SetFontSize(size))
	}

	fn set_font_color(&'a mut self, color: Color) -> &mut EntityCommands<'a> {
		self.add(SetFontColor(color))
	}
}

pub struct SetFont(String);

impl EntityCommand for SetFont {
	fn apply(self, entity: Entity, world: &mut World) {
		let asset_server = world.resource::<AssetServer>();
		let font = asset_server.load(&self.0);

		if let Some(mut text) = world.entity_mut(entity).get_mut::<Text>() {
			for text_section in &mut text.sections {
				text_section.style.font = font.clone();
			}
		}
	}
}

pub struct SetFontSize(f32);

impl EntityCommand for SetFontSize {
	fn apply(self, entity: Entity, world: &mut World) {
		if let Some(mut text) = world.entity_mut(entity).get_mut::<Text>() {
			for text_section in &mut text.sections {
				text_section.style.font_size = self.0;
			}
		}
	}
}

pub struct SetFontColor(Color);

impl EntityCommand for SetFontColor {
	fn apply(self, entity: Entity, world: &mut World) {
		if let Some(mut text) = world.entity_mut(entity).get_mut::<Text>() {
			for text_section in &mut text.sections {
				text_section.style.color = self.0;
			}
		}
	}
}
