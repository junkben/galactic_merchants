use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Resource)]
pub enum Difficulty {
	#[default]
	Tutorial,
	Novice,
	Beginner,
	Intermediate,
	Expert,
	Master
}
