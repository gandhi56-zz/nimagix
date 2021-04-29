use bevy::prelude::*;

mod screens;
mod gamedata;
mod gamestate;

use crate::screens::ScreensPlugin;
use crate::gamedata::GameData;
use crate::gamestate::GameState;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor{
            title: "Nimagix".to_string(),
            width: 900.0,
            height: 700.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.61, 0.72, 0.96)))
        .insert_resource(GameData{
            game_state: GameState::Menu,
            score: 0,
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_plugin(ScreensPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
