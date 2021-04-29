
#[derive(std::cmp::PartialEq)]
pub struct GameState{
    Menu,
    Playing,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin{
    fn build(&self, app: &mut AppBuilder){
        // app.add_system()
    }
}

fn handle_gamestate_system(
    mut game_data: ResMut<GameData>,
    keyboard_input: Res<Input<KeyCode>>,
){

}