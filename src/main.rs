mod poker;

use poker::card::{Card, Rank, Suit};
use poker::hand::Hand;

fn main() {
    let player_hand = Hand::new(vec![
        Card::new(Suit::Spades, Rank::Ace),
        Card::new(Suit::Spades, Rank::King),
    ]);

    println!("Player's hand: {:?}", player_hand);
}
