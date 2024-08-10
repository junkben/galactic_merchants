use bevy::prelude::*;

macro_rules! sound_plugin {
    ({
		plugin: $plugin:ident,
		event: $event:ident,
		read: $read:ident,
		sounds: [$({
            marker: $marker:ident,
            subevent: $subevent:ident,
            path: $path:expr
        }),*]
	}) => (
        pub struct $plugin;

        impl Plugin for $plugin {
            fn build(&self, app: &mut App) {
                app
                    .add_event::<$event>()
                    .add_systems(Update, $read.run_if(on_event::<$event>()));
            }
        }

		#[derive(Event)]
		pub enum $event {
			$($subevent),*
		}

        $(
            #[derive(Component)]
            struct $marker;
		)*

		fn $read(
			mut commands: Commands,
			mut er: EventReader<$event>,
			asset_server: Res<AssetServer>
		) {
			let Some(event) = er.read().last() else {
				return
			};

			match event {
				$(
					$event::$subevent => {
						trace!("playing {}", stringify!($marker));
						commands.spawn((
							AudioBundle {
								source: asset_server.load($path),
								settings: PlaybackSettings {
									mode: bevy::audio::PlaybackMode::Despawn,
									..default()
								},
								..default()
							},
							$marker,
						));
					}
				),*
			}
		}
    )
}

sound_plugin!({
	plugin: MenuAudioPlugin,
	event: PlayMenuSound,
	read: handle_event_play_menu_sound,
	sounds: [
		{
			marker: SoundButtonHovered,
			subevent: ButtonHovered,
			path: "sounds/ui/button-hover.mp3"
		},
		{
			marker: SoundButtonPressed,
			subevent: ButtonPressed,
			path: "sounds/ui/button-press.mp3"
		}
	]
});
