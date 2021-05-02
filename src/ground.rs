
use crate::SCREEN_WIDTH;
use bevy::prelude::*;

pub struct GroundPlugin;

impl Plugin for GroundPlugin{
    fn build(&self, app: &mut AppBuilder){
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let tile_size = Vec2::splat(8.);
    let tile_scale = Vec2::splat(6.);
    let texture_handle = assets.load("graphics/tile_set.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 34, 28);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    let start_pos = Vec2::new(-SCREEN_WIDTH, -500. + tile_size.y);
    let mut pos = Vec2::new(start_pos.x, start_pos.y);
    while pos.y + (tile_size.y * tile_scale.y) <= -200.0{
        pos.x = start_pos.x;
        while pos.x + (tile_size.x * tile_scale.x) <= SCREEN_WIDTH{
            commands.spawn_bundle(SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                transform: Transform{
                    translation: Vec3::new(pos.x, pos.y, 0.0),
                    scale: tile_scale.extend(0.0),
                    ..Default::default()
                },
                ..Default::default()
            });
            pos.x += tile_size.x * tile_scale.x;
        }
        pos.y += tile_scale.y * tile_size.y;
    }
}