use bevy::prelude::*;
use bevy_inspector_egui::prelude::*;

pub const ARENA_WIDTH: u32 = 15;
pub const ARENA_HEIGHT: u32 = 15;

pub const SNAKE_HEAD_COLOR: Color = Color::srgb(0.80, 0.49, 0.12);
pub const SNAKE_SEGMENT_COLOR: Color = Color::srgb(0.43, 0.30, 0.15);
pub const APPLE_COLOR: Color = Color::srgb(1.0, 0.0, 0.0);
pub const WALL_COLOR: Color = Color::srgb(1., 1., 1.);

#[derive(Component)]
pub struct Size {
	pub height: f32,
	pub width: f32
}
impl Size {
	pub fn squaire(x: f32) -> Self {
		Size {
			height: x,
			width: x
		}
	}
}

#[derive(Component, Reflect, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(PartialEq, Copy, Clone)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}
impl Direction {
    pub fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(Component)]
pub struct Cooldown {
	pub timer: Timer
}

// Snake 
#[derive(Component)]
pub struct SnakeHead {
	pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Resource)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Resource)]
pub struct LastTailPosition(pub Option<Position>);

// Food 
#[derive(Event)]
pub struct GrowthEvent;

#[derive(Component)]
pub struct Food;

// walls

#[derive(Reflect, Resource, Default, InspectorOptions)]
#[reflect(Resource, InspectorOptions)]
pub struct Rules {
	pub pause: bool,

	pub next: Option<Position>,

	pub spawn_obstruction_timer: Timer,
	pub spawn_food_timer: Timer,

	#[inspector(min = 0, max = 85)]
	pub obstruction_at_once: u32,

	#[inspector(min = 0, max = 15)]
	pub apples_at_once: u32,
}

#[derive(Component)]
pub struct Obstruction;