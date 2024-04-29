use crate::prelude::*;

#[derive(Resource)]
pub struct MovementTimer(pub Timer);

#[derive(Resource, Default)]
pub enum MovementDirection {
    Up,
    #[default]
    Right,
    Down,
    Left,
}

#[derive(Resource, Default)]
pub enum MovementState {
    #[default]
    Idle,
    // Crouch(MovementDirection), //TODO
}
