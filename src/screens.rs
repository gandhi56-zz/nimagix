
use bevy::sprite::entity::SpriteBundle;
use bevy::prelude::*;

pub struct StartScreen;
pub struct PlayingScreen;
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

    // render start screen ------------------------------------------------------ //
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(SpriteBundle{
            material: materials.add(game_title_texture.into()),
            transform: Transform{
                translation: Vec3::new(0.0, 200.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(StartScreen);
    commands
        .spawn_bundle(TextBundle {
            transform: Transform{
                translation: Vec3::new(100.0, 200.0, 0.0),
                ..Default::default()
            },
            text: Text::with_section(
                "Press SPACE to start game!",
                TextStyle {
                    font: asset_server.load("fonts/Fixedsys500c.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
                TextAlignment {
                    horizontal: HorizontalAlign::Center,
                    ..Default::default()
                },
            ),
            ..Default::default()
        })
        .insert(StartScreen);

    commands.spawn()
        .insert(PlayingScreen);
}