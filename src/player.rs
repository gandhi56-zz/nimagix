use bevy::prelude::*;

pub struct Player{
    state: PlayerState,
    frame_idx: usize,
}

#[derive(Debug)]
pub enum PlayerState{
    IdleLeft,
    IdleRight,
    MovingRight,
    MovingLeft,
    // TODO #3 add more states like jumping, flying, growing, etc.
}

impl Player{
    pub fn new() -> Player{
        Player{state: PlayerState::IdleRight, frame_idx: 0}
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin{
    fn build(&self, app: &mut AppBuilder){
        app.add_startup_system(setup.system())
            .add_system(keyboard_player_system.system())
            .add_system(animate_player.system());
    }
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let tile_size = Vec2::splat(12.);
    let tile_scale = Vec2::new(4.0, 3.0);
    let texture_handle = assets.load("graphics/smb_mario_sheet.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, tile_size, 14, 12);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    
    // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle{
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(tile_scale.extend(0.0)),
        ..Default::default()
    })
    .insert(Timer::from_seconds(0.1, true))
    .insert(Player::new());
}

fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<bevy::prelude::TextureAtlas>)>,
    mut player: ResMut<Player>,
){
    // sprite indices
    let idle_right_sprites = [7u32];
    let idle_left_sprites = [6u32];
    let moving_right_sprites = [8u32, 9, 10, 11];
    let moving_left_sprites = [5u32, 4, 3, 2];

    for (mut timer, mut sprite, _texture_atlas_handle) in query.iter_mut(){
        timer.tick(time.delta());
        if timer.finished(){
            match &player.state{
                PlayerState::IdleRight => {
                    sprite.index = idle_right_sprites[0];
                }
                PlayerState::IdleLeft => {
                    sprite.index = idle_left_sprites[0];
                }
                PlayerState::MovingRight => {
                    sprite.index = moving_right_sprites[player.frame_idx];
                    player.frame_idx = (player.frame_idx + 1) % moving_right_sprites.len();
                }
                PlayerState::MovingLeft => {
                    sprite.index = moving_left_sprites[player.frame_idx];
                    player.frame_idx = (player.frame_idx + 1) % moving_left_sprites.len();
                }
            }
        }
    }
}

fn keyboard_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: ResMut<Player>,
){
    if keyboard_input.pressed(KeyCode::D){
        player.state = PlayerState::MovingRight;
    }
    else if keyboard_input.pressed(KeyCode::A){
        player.state = PlayerState::MovingLeft;
    }
    else if keyboard_input.just_released(KeyCode::D){
        player.state = PlayerState::IdleRight;
        player.frame_idx = 0;
    }
    else if keyboard_input.just_released(KeyCode::A){
        player.state = PlayerState::IdleLeft;
        player.frame_idx = 0;
    }
}