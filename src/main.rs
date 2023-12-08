mod poker;
use poker::deck::Deck;

fn main() {
    let mut deck = Deck::new();
    println!("Deck: {:?}", deck);
    
    deck.shuffle();
    println!("Shuffled: {:?}", deck);

    let Some(card) = deck.draw() else { todo!() };
    println!("Drawn: {:?}", card);

    println!("Left: {}", deck.cards.len());
}
