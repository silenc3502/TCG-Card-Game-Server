use serde::{Deserialize, Serialize};
use crate::game_hand::controller::response_form::mulligan_response_form::MulliganResponseForm;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameDeckCardRedrawResponse {
    redrawn_card_list: Vec<i32>,
    remaining_shuffled_deck: Vec<i32>
}

impl GameDeckCardRedrawResponse {
    pub fn new(redrawn_card_list: Vec<i32>, remaining_shuffled_deck: Vec<i32>) -> Self {
        GameDeckCardRedrawResponse {
            redrawn_card_list,
            remaining_shuffled_deck
        }
    }
    pub fn to_change_first_hand_response_form(&self) -> MulliganResponseForm {
        MulliganResponseForm::new(
            self.redrawn_card_list.clone(),
            self.remaining_shuffled_deck.clone()
        )
    }
}