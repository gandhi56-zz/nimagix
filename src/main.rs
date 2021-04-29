use bevy::prelude::*;

fn main() {
    App::build()
        .insert_resource(WindowDescriptor{
            title: "Nimagix".to_string(),
            width: 900.0,
            height: 700.0,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>
){
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
