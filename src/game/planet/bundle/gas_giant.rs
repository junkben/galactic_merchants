use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::shaders::clouds::{CloudColors, CloudConfig, CloudShader};

pub struct SpawnGasGiant {
	pub transform:   Transform,
	pub inner_cloud: CloudShader,
	pub outer_cloud: CloudShader
}

impl Default for SpawnGasGiant {
	/// from GasPlanet.tscn
	fn default() -> Self {
		SpawnGasGiant {
			transform:   Transform::default().with_scale(Vec3::splat(400.0)),
			inner_cloud: CloudShader {
				config: CloudConfig {
					pixels:         100.0,
					cloud_cover:    0.0,
					light_origin:   Vec2 { x: 0.25, y: 0.25 },
					time_speed:     0.7,
					stretch:        1.0,
					cloud_curve:    1.3,
					light_border_1: 0.692,
					light_border_2: 0.666,
					rotation:       0.0,
					size:           9.0,
					octaves:        5,
					seed:           5.939
				},
				colors: CloudColors {
					base:           LinearRgba::rgb(0.23, 0.13, 0.15),
					outline:        LinearRgba::rgb(0.23, 0.13, 0.15),
					shadow_base:    LinearRgba::rgb(0.13, 0.09, 0.11),
					shadow_outline: LinearRgba::rgb(0.13, 0.09, 0.11)
				}
			},
			outer_cloud: CloudShader {
				config: CloudConfig {
					pixels:         100.0,
					cloud_cover:    0.538,
					light_origin:   Vec2 { x: 0.25, y: 0.25 },
					time_speed:     0.47,
					stretch:        1.0,
					cloud_curve:    1.3,
					light_border_1: 0.439,
					light_border_2: 0.746,
					rotation:       0.0,
					size:           9.0,
					octaves:        5,
					seed:           5.939
				},
				colors: CloudColors {
					base:           LinearRgba::rgb(0.94, 0.71, 0.25),
					outline:        LinearRgba::rgb(0.81, 0.46, 0.17),
					shadow_base:    LinearRgba::rgb(0.67, 0.32, 0.19),
					shadow_outline: LinearRgba::rgb(0.49, 0.22, 0.20)
				}
			}
		}
	}
}

impl SpawnGasGiant {
	fn inner_cloud_transform(&self) -> Transform {
		self.transform.with_translation(Vec3 {
			x: self.transform.translation.x,
			y: self.transform.translation.y,
			z: self.transform.translation.z - 1.0
		})
	}

	fn outer_cloud_transform(&self) -> Transform { self.transform }

	pub fn to_sprite_bundles(
		&self,
		meshes: &mut Assets<Mesh>,
		materials: &mut Assets<CloudShader>
	) -> Vec<MaterialMesh2dBundle<CloudShader>> {
		vec![
			MaterialMesh2dBundle {
				mesh: meshes.add(Rectangle::default()).into(),
				transform: self.inner_cloud_transform(),
				material: materials.add(self.inner_cloud.clone()),
				..default()
			},
			MaterialMesh2dBundle {
				mesh: meshes.add(Rectangle::default()).into(),
				transform: self.outer_cloud_transform(),
				material: materials.add(self.outer_cloud.clone()),
				..default()
			},
		]
	}
}
