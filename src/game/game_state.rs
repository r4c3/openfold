use crate::poker::card::Card
use super::player::Player
use super::bet_options::BetOptions

#[derive(Debug, Clone)]
pub struct GameState {
    board: Vec<Card>,
    pot_size: u16,
    street: Street,
    players: [Player; 2],
    settings: GameSettings,
}

#[derive(Debug, Clone)]
pub struct GameSettings {
    rake_percentage: u8,
    rake_cap: u8,
    add_all_in_threshold: u8,
    force_all_in_threshold: u8,
    merging_threshold: u8,
    bet_options: BetOptions,
}
