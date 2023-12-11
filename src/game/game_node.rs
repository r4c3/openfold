use super::street::Street
use crate::poker::Card

#[derive(Debug, Clone, PartialEq)]
pub enum PlayerAction {
    Fold,
    Check,
    Call,
    Bet(u16),
    Raise(u16),
}

#[derive(Debug, Clone)]
pub struct GameNode {
    street: Street,
    board: Vec<Card>,
    pot_size: u16,
    actions: Vec<PlayerAction>,
    children: Vec<GameNode>,
}

impl GameNode {
    pub fn new(street: Street, board: Vec<Card>, pot_size: u16, actions: Vec<PlayerAction>) -> Self {
        GameNode {
            street, board, pot_size, actions,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: GameNode) {
        self.children.push(child);
    }

    pub fn expand(&mut self) {
        if self.is_terminal() { return; }

        let available_actions = self.determine_available_actions();

        for action in available_actions {
            let mut child_state = self.clone();
            child_state.apply_action(action);
            child_state.advance();

            self.children.push(GameNode::new(
                child_state.street,
                child_state.board,
                child_state.pot_size,
                child_state.actions,
            ))
        }

        if self.should_advance_street() {
            self.deal_next_street();
        }
    }

    pub fn is_terminal(&self) -> bool {
        unimplemented!()
    }

    pub fn determine_available_actions(&self) -> Vec<PlayerAction> {
        unimplemented!()
    }
    
    pub fn apply_action(&mut self, action: PlayerAction) {
        unimplemented!()
    }

    pub fn advance (&mut self) {
        unimplemented!()
    }

    pub fn should_advance_street(&self) -> bool {
        unimplemented!()
    }

    pub fn deal_next_street(&mut self) {
        unimplemented!()
    }
}
