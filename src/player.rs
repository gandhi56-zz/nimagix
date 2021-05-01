use bevy::prelude::*;

pub struct Player;
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut AppBuilder){
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let tile_size = Vec2::splat(12.);
    let tile_scale = Vec2::splat(6.);
    let texture_handle = assets.load("graphics/smb_mario_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 14, 6);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(tile_scale.extend(0.0)),
        ..Default::default()
    });
}