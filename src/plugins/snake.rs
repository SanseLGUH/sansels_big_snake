use crate::components::*;
use bevy::prelude::*;

pub fn spawn_snake(mut commands: Commands, mut segments: ResMut<SnakeSegments>) {
	*segments = SnakeSegments(vec![
		commands.spawn((
			Sprite::from_color(SNAKE_HEAD_COLOR, Vec2::new(1., 1.)),
			Position { x: 2, y: 2 }, Size::squaire(0.8), SnakeHead { direction: Direction::Up }, 
			Cooldown { timer: Timer::from_seconds(0.3, TimerMode::Repeating) }
		)).id(),
		spawn_segment(&mut commands, Position {x: 2, y: 1})
	]);
}

fn spawn_segment(commands: &mut Commands, position: Position) -> Entity {
	commands.spawn((
		Sprite::from_color(SNAKE_SEGMENT_COLOR, Vec2::new(1., 1.)),
		SnakeSegment, Obstruction, position, Size::squaire(0.65), 
	)).id()
}

pub fn snake_growth(
    mut commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
) {
    for _ in growth_reader.read() {
        segments.0.push(spawn_segment(&mut commands, last_tail_position.0.unwrap())); 
    }
}

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_die(
    obstruction_positions: Query<&Position, With<Obstruction>>,
    head_positions: Query<&Position, With<SnakeHead>>,
    rules: Res<Rules>
    ) {
    if !rules.pause {
        for obs_position in obstruction_positions {
            for head_pos in head_positions {
                if obs_position == head_pos {
                    std::process::exit(0);
                }
            }
        }
    }
}

pub fn snake_movement(
    rules: Res<Rules>,
    time: Res<Time>, 
    mut heads: Query<(Entity, &SnakeHead, &mut Cooldown)>, 
    segments: ResMut<SnakeSegments>, mut last_tail_position: ResMut<LastTailPosition>,
    mut positions: Query<&mut Position>) {
    for (head_entity, head, mut cooldown) in &mut heads {
        if cooldown.timer.tick(time.delta()).finished() {
	        let segment_positions = segments.0
	            .iter()
	            .map(|e| *positions.get_mut(*e).unwrap())
	            .collect::<Vec<Position>>();
	        let mut head_pos = positions.get_mut(head_entity).unwrap();

            *last_tail_position = LastTailPosition(Some(*segment_positions.last().unwrap()));

            if (head_pos.x == ARENA_WIDTH as i32 || head_pos.y == ARENA_HEIGHT as i32) ||
               (head_pos.x == -1 || head_pos.y == -1) {
                std::process::exit(0);
            }

            if !rules.pause {
                match head.direction {
                    Direction::Left => head_pos.x -= 1,
                    Direction::Right => head_pos.x += 1,
                    Direction::Up => head_pos.y += 1,
                    Direction::Down => head_pos.y -= 1,
                }

                segment_positions
                    .iter()
                    .zip(segments.0.iter().skip(1))
                    .for_each(|(pos, segment)| {
                        *positions.get_mut(*segment).unwrap() = *pos;
                    });
            }
        }
    }
}

pub fn snake_movement_input(keyboard_input: Res<ButtonInput<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::ArrowLeft) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::ArrowDown) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::ArrowUp) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            Direction::Right
        } else {
            head.direction
        };
        if dir != head.direction.opposite() {
            head.direction = dir;
        }
    }
}