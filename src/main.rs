use bevy::prelude::*;

mod screens;

use crate::screens::ScreensPlugin;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor{
            title: "Nimagix".to_string(),
            width: 900.0,
            height: 700.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.61, 0.72, 0.96)))
        .add_plugins(DefaultPlugins)
        .add_plugin(ScreensPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
