
use bevy::app::AppBuilder;
use bevy::app::Plugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState{
    Menu,
    Playing,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin{
    fn build(&self, _app: &mut AppBuilder){

    }
}
