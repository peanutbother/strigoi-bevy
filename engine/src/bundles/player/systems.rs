use std::f32::consts::PI;

use super::{bindings::*, components::*};
use crate::prelude::*;

pub fn select_level_from_pos(
    players: Query<&GlobalTransform, With<Player>>,
    levels: Query<(&LevelIid, &GlobalTransform)>,
    ldtk_project: Query<&Handle<LdtkProject>>,
    ldtk_project_assets: Res<Assets<LdtkProject>>,
    mut level_selection: ResMut<LevelSelection>,
) {
    if let Ok(player_transform) = players.get_single() {
        let ldtk_project = ldtk_project_assets
            .get(ldtk_project.single())
            .expect("ldtk project should be loaded before player is spawned");

        for (level_iid, level_transform) in levels.iter() {
            let level = ldtk_project
                .get_raw_level_by_iid(level_iid.get())
                .expect("level should exist in project");
            let level_bounds = Rect {
                min: Vec2::new(
                    level_transform.translation().x,
                    level_transform.translation().y,
                ),
                max: Vec2::new(
                    level_transform.translation().x + level.px_wid as f32,
                    level_transform.translation().y + level.px_hei as f32,
                ),
            };

            if level_bounds.contains(player_transform.translation().truncate()) {
                *level_selection = LevelSelection::Iid(level_iid.clone());
            }
        }
    }
}
pub fn move_player_from_input(
    mut players: Query<(&mut GridCoords, &mut TextureAtlasSprite, &mut Transform), With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut timer: ResMut<MovementTimer>,
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let movement_direction = if input.pressed(KeyCode::W) {
        (GridCoords::new(0, 1), MovementDirection::Up)
    } else if input.pressed(KeyCode::A) {
        (GridCoords::new(-1, 0), MovementDirection::Left)
    } else if input.pressed(KeyCode::S) {
        (GridCoords::new(0, -1), MovementDirection::Down)
    } else if input.pressed(KeyCode::D) {
        (GridCoords::new(1, 0), MovementDirection::Right)
    } else {
        return;
    };

    for (mut player_grid_coords, mut sprite, mut transform) in players.iter_mut() {
        let destination = *player_grid_coords + movement_direction.0;
        *player_grid_coords = destination;

        match movement_direction.1 {
            MovementDirection::Up => {
                transform.rotation = Quat::from_rotation_z(90.0 * (PI / 180.0));
            }
            MovementDirection::Down => {
                transform.rotation = Quat::from_rotation_z(0.0);
            }
            MovementDirection::Right => {
                sprite.flip_x = false;
                transform.rotation = Quat::from_rotation_z(0.0);
            }
            MovementDirection::Left => {
                sprite.flip_x = true;
                transform.rotation = Quat::from_rotation_z(0.0);
            }
        };
    }
}

pub fn translate_grid_coords_entities(
    mut grid_coords_entities: Query<(&mut Transform, &GridCoords), Changed<GridCoords>>,
) {
    for (mut transform, grid_coords) in grid_coords_entities.iter_mut() {
        transform.translation = bevy_ecs_ldtk::utils::grid_coords_to_translation(
            *grid_coords,
            IVec2::splat(TILEMAP_GRID_SIZE),
        )
        .extend(transform.translation.z);
    }
}
