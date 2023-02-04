use bevy::prelude::Reflect;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Reflect)]
pub enum MainGameState {
    MENU,
    COMBAT,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Copy, Reflect)]
pub enum ProjectilePresent {
    PRESENT,
    NA,
}
