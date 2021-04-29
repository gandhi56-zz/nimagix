
use bevy::ecs::system::ResMut;
use bevy::ecs::system::Res;
use bevy::input::Input;
use bevy::input::keyboard::KeyCode;
use crate::GameData;
use bevy::app::AppBuilder;
use bevy::app::Plugin;
use bevy::prelude::*;

pub enum GameState{
    Menu,
    Playing,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin{
    fn build(&self, app: &mut AppBuilder){
        app.add_system(handle_gamestate.system());
    }
}

fn handle_gamestate(
    mut game_data: ResMut<GameData>,
    keyboard_input: Res<Input<KeyCode>>,
){

}