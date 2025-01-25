use bevy::prelude::*;

#[derive(Event, Default)]
pub struct PlayerLostEvent;

// This event is for the ui to update the progression of the player
#[derive(Event)]
pub struct TimePassedEvent;
