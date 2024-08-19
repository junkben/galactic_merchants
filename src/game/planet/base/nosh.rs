use bevy::prelude::*;

use crate::{
	game::planet::bundle::gas_giant::SpawnGasGiant,
	shaders::clouds::{CloudColors, CloudConfig, CloudShader}
};

#[derive(Event)]
pub struct SpawnNosh {
	transform: Transform
}

#[derive(Component)]
pub struct IsNosh;

pub fn handle_event_spawn_nosh(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<CloudShader>>,
	mut er: EventReader<SpawnNosh>
) {
	let Some(event) = er.read().last() else {
		return;
	};

	let mut nosh = nosh_spawn();
	nosh.transform = event.transform;

	for bundle in nosh.to_sprite_bundles(meshes.as_mut(), materials.as_mut()) {
		commands.spawn((bundle, super::IsBasePlanet, IsNosh));
	}
}

fn nosh_spawn() -> SpawnGasGiant {
	SpawnGasGiant {
		transform:   Default::default(),
		inner_cloud: CloudShader {
			config: CloudConfig {
				pixels:         128.0,
				cloud_cover:    0.0,
				light_origin:   Vec2 { x: 0.25, y: 0.25 },
				time_speed:     0.1,
				stretch:        1.0,
				cloud_curve:    1.3,
				light_border_1: 0.7,
				light_border_2: 0.85,
				rotation:       0.0,
				size:           15.0,
				octaves:        5,
				seed:           5.939
			},
			colors: CloudColors {
				base:           LinearRgba::from_u8_array([101, 0, 0, 255]),
				outline:        LinearRgba::from_u8_array([101, 0, 0, 255]),
				shadow_base:    LinearRgba::from_u8_array([79, 3, 1, 255]),
				shadow_outline: LinearRgba::from_u8_array([79, 3, 1, 255])
			}
		},
		outer_cloud: CloudShader {
			config: CloudConfig {
				pixels:         128.0,
				cloud_cover:    0.35,
				light_origin:   Vec2 { x: 0.25, y: 0.25 },
				time_speed:     0.47,
				stretch:        1.0,
				cloud_curve:    1.3,
				light_border_1: 0.7,
				light_border_2: 0.85,
				rotation:       0.0,
				size:           15.0,
				octaves:        5,
				seed:           5.939
			},
			colors: CloudColors {
				base:           LinearRgba::from_u8_array([246, 152, 0, 255]),
				outline:        LinearRgba::from_u8_array([201, 108, 0, 255]),
				shadow_base:    LinearRgba::from_u8_array([171, 72, 7, 255]),
				shadow_outline: LinearRgba::from_u8_array([85, 12, 0, 255])
			}
		}
	}
}
