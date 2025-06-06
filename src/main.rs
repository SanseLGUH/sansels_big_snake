mod components;
mod plugins;

use bevy::{prelude::*, window::*};
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use bevy_egui::EguiPlugin;
use crate::{plugins::*, components::*};

fn size_scaling(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut q: Query<(&Size, &mut Transform)>
) {
    let window = windows.single().unwrap(); // This will panic if there's not exactly one primary window

    for (sprite_size, mut transform) in q.iter_mut() {
        transform.scale = Vec3::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width(),
            sprite_size.height / ARENA_HEIGHT as f32 * window.height(),
            1.,
        );
    }
}

fn position_translation(windows: Query<&Window, With<PrimaryWindow>>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }

    let window = windows.single().unwrap();

    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

use rand::random;

fn next_object_position(mut rules: ResMut<Rules>, positions: Query<&Position>) {
    let mut change = false;

    match rules.next {
        Some(old_next) => {
            for position in positions {
                if *position == old_next {
                    change = true;
                }
            }  
        }
        None => {
            change = true;
        }
    }

    if change {
        loop {
            let obj_position = Position {
                x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
                y: (random::<f32>() * ARENA_HEIGHT as f32) as i32
            };

            let mut next_bool = true;

            for position in positions {
                if *position == obj_position {
                    next_bool = false;
                }
            }

            if next_bool {
                rules.next = Some(obj_position);
                
                break;
            }
        }  
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: String::from("Sansel's Big Snake!"),
                resolution: WindowResolution::new(500., 500.),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_camera).add_systems(Update, simple_pause)
        .add_plugins(Snake).add_plugins(Foods).add_plugins(Obstructions)
        .add_systems( PostUpdate, ( position_translation, size_scaling ) )
        .add_systems(Update, next_object_position)
        .insert_resource(SnakeSegments(Vec::new()))
        .insert_resource( Rules {
            pause: true,

            next: None,

            spawn_obstruction_timer: Timer::from_seconds(15., TimerMode::Repeating),
            spawn_food_timer: Timer::from_seconds(3., TimerMode::Repeating),

            obstruction_at_once: 25,
            apples_at_once: 1,
        } )
        .register_type::<Rules>()
        .add_plugins(EguiPlugin { enable_multipass_for_primary_context: true })
        .add_plugins(ResourceInspectorPlugin::<Rules>::default())

        .insert_resource(ClearColor (Color::srgb(0.04, 0.04, 0.04)) )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn simple_pause( keyboard_input: Res<ButtonInput<KeyCode>>, mut rules: ResMut<Rules> ) {
    if keyboard_input.pressed(KeyCode::Escape) {
        rules.pause = !rules.pause;
    }
}