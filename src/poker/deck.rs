use super::card::{Rank, Suit, Card};

use rand::seq::SliceRandom;
use rand::thread_rng;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::with_capacity(52);
        for suit in Suit::iter() {
            for rank in Rank::iter() {
                cards.push(Card::new(suit, rank))
            }
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}
