use crate::gamestate;
use gamestate::GameState;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub struct GameData {
    pub state: GameState,
    pub score: i32,
}
