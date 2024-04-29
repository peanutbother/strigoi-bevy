use bevy_mod_picking::PickableBundle;

use crate::prelude::*;

pub const IDENTIFIER: &str = "Player";

#[derive(Default, Component)]
pub struct Player;

#[derive(Default, Bundle, LdtkEntity)]
pub struct PlayerBundle {
    player: Player,
    #[sprite_sheet_bundle]
    sprite_bundle: SpriteSheetBundle,
    #[grid_coords]
    grid_coords: GridCoords,
    #[worldly]
    worldly: Worldly,
}

pub(crate) fn process_entity(mut commands: Commands, new_players: Query<Entity, Added<Player>>) {
    for player in new_players.iter() {
        commands.spawn(PickableBundle::default()).set_parent(player);
    }
}
