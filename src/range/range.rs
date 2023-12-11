const HAND_COMBINATIONS: usize = 52 * 51 / 2;

#[derive(Debug, Clone, PartialEq)]
pub struct Range {
    range: [u16; HAND_COMBINATIONS],
}

impl Range {
    pub fn new() -> Self {
        Range {
            range: [0; HAND_COMBINATIONS],
        }
    }

    fn cards_to_index(&self, card1: usize, card2: usize) -> usize {
        assert!(card1 < 52 && card2 < 52, "Card index OOB.");
        assert!(card1 != card2, "Duplicate cards disallowed.");

        let (min_card, max_card) = if card1 < card 2 { (card1, card2) } else { (card2, card1) };

        min_card * 51 + max_card - (min_card * (min_card + 1) / 2)
    }

    fn normalize(&mut self) {
        let total: u32 = self.data.iter().map(|&p| p as u32).sum();
        if total == 0 { return; }

        self.data.iter_mut().for_each(|p| *p = ((*p as u32 * 100) / total) as u8);
    }

    fn is_valid(&self) -> bool {
        self.data.iter().all(|&p| p <= 100)
    }
}
