use bevy::prelude::*;
use crate::components::*;

mod snake;
mod objects;

pub struct Snake;
impl Plugin for Snake {
	fn build(&self, app: &mut App) {
		app
			.add_systems(Startup, snake::spawn_snake)
			.add_systems(Update, snake::snake_eating)
			.add_systems(Update, snake::snake_die)
			.add_systems(Update, snake::snake_movement)
			.add_systems(Update, snake::snake_movement_input.before(snake::snake_movement))
			.add_systems(Update, snake::snake_growth.after(snake::snake_eating))
			.insert_resource(LastTailPosition(None))
			.add_event::<GrowthEvent>();
	}
}

pub struct Foods;
impl Plugin for Foods {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, objects::apple_spawner);
	}
}

pub struct Obstructions;
impl Plugin for Obstructions {
	fn build(&self, app: &mut App) {
		app.add_systems(Update, objects::wall_spawner);
	}
}