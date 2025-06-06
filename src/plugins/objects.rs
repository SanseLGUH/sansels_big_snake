use bevy::prelude::*;
use crate::components::*;

pub fn apple_spawner(mut commands: Commands, time: Res<Time>, mut rules: ResMut<Rules>, foods: Query<&Food> ) {
	if !rules.pause && rules.apples_at_once > foods.iter().count() as u32 && rules.spawn_food_timer.tick(time.delta()).finished() {
		if let Some(position) = rules.next {
			commands.spawn((
				Sprite::from_color(APPLE_COLOR, Vec2::new(1., 1.)),
				Food, Size::squaire(0.8), position
			));
		} 
	} 
}

// 
pub fn wall_spawner( mut commands: Commands, time: Res<Time>, mut rules: ResMut<Rules>, walls: Query<&Obstruction> ) {
	if !rules.pause && rules.obstruction_at_once > walls.iter().count() as u32 && rules.spawn_obstruction_timer.tick(time.delta()).finished() {
		if let Some(position) = rules.next {
			commands.spawn((
				Sprite::from_color(WALL_COLOR, Vec2::new(1., 1.)),
				Obstruction, Size::squaire(0.8), position
			));
		}
	}
}