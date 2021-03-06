use crate::player::{PlayerPlugin, Player};
use bevy::window::WindowMode;
use bevy::prelude::*;

mod screens;
mod gamedata;
mod gamestate;
mod ground;
mod player;

use crate::ground::GroundPlugin;
use crate::gamestate::*;
use crate::screens::ScreensPlugin;
use crate::gamedata::GameData;

const SCREEN_WIDTH: f32 = 1280.0;
const SCREEN_HEIGHT: f32 = 720.0;

fn main() {
    App::build()
        // insert all plugins
        .add_plugins(DefaultPlugins)
        .add_plugin(GroundPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(ScreensPlugin)
        .add_plugin(GameStatePlugin)
        
        .add_startup_system(setup.system())
        
        // insert resources
        .insert_resource(WindowDescriptor{
            vsync: true,
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            title: "Nimagix".to_string(),
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.51, 0.62, 0.86)))
        .insert_resource(GameData{
            state: GameState::Menu,
            score: 0,
        })
        .insert_resource(Player::new())
        .run();
}

fn setup(
    mut commands: Commands,
){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
