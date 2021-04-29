
use bevy::sprite::entity::SpriteBundle;
use bevy::prelude::*;

pub struct StartScreen;
pub struct ScreensPlugin;

impl Plugin for ScreensPlugin{
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    // load screen assets here
    let game_title_texture = asset_server.load("graphics/title_screen.png");

    // render start screen
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.add(game_title_texture.into()),
            ..Default::default()
        })
        .insert(StartScreen);
}