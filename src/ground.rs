use crate::SCREEN_HEIGHT;
use crate::SCREEN_WIDTH;
use bevy::window::WindowResized;
use bevy::app::Events;
use bevy::prelude::*;

pub struct GroundPlugin;

impl Plugin for GroundPlugin{
    fn build(&self, app: &mut AppBuilder){
        app.add_startup_system(setup.system());
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    // spawn a 2D camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // spawn a tilemap of bricks for ground
    let mut xOffset: f32 = 0.0;
    let tile_size = 8.;
    let texture_handle = asset_server.load("graphics/tile_set.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, 
        Vec2::new(tile_size, tile_size), 34, 28);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    // let vendor_idx = texture_atlas.get_texture_index(&texture_handle).unwrap();
    
    // for y in 0..(SCREEN_HEIGHT / tile_size - 1.0) as u32{
    //     for x in 0..(SCREEN_WIDTH / tile_size - 1.0) as u32{
    //         let pos = Vec3::new((x as f32) * tile_size, (y as f32) * tile_size, 0.0);
            
    //     }
    // }

}