use super::game_state::Street

#[derive(Debug, Clone)]
pub struct BetOptions {
    flop_bet_sizes: Vec<u8>,
    turn_bet_sizes: Vec<u8>,
    river_bet_sizes: Vec<u8>,
}

impl BetOptions {
    pub fn new(flop_bet_sizes: Vec<u8>, turn_bet_sizes: Vec<u8>, river_bet_sizes<u8>) -> Self {
        BetOptions {
            flop_bet_sizes,
            turn_bet_sizes,
            river_bet_sizes,
        }
    }

    pub fn get_bet_options(&self, street: &Street) -> &[u8] {
        match street {
            Street::Flop => &self.flop_bet_sizes,
            Street::Turn => &self.turn_bet_sizes,
            Street::River => &self.river_bet_sizes,
        }
    }
}
