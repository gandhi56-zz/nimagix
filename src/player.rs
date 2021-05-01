use bevy::prelude::*;

pub struct Player<'a>{
    name: &'a str,
    state: PlayerState,
}

pub enum PlayerState{
    Idle_Left,
    Idle_Right,
    Moving_Right,
    Moving_Left,
    // TODO #3 add more states like jumping, flying, growing, etc.
}

impl Player<'static>{
    pub fn new() -> Player<'static>{
        Player{name: "mario", state: PlayerState::Idle_Right}
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut AppBuilder){
        app.add_startup_system(setup.system())
            .add_system(animate_player.system());
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
    
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(tile_scale.extend(0.0)),
        ..Default::default()
    })
    .insert(Timer::from_seconds(0.1, true));
}

fn animate_player(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>
){
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut(){
        timer.tick(time.delta());
        if timer.finished(){
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = ((sprite.index as usize + 1) % texture_atlas.textures.len()) as u32;
        }
    }
}