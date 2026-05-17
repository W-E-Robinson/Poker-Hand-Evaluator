mod validate_no_repeated_cards;
mod validate_number_board_cards;
mod validate_number_burn_cards;
mod validate_number_player_cards;
mod validate_number_total_cards;
mod validate_player_ids;

use self::{
    validate_no_repeated_cards::validate_no_repeated_cards,
    validate_number_board_cards::validate_number_board_cards,
    validate_number_burn_cards::validate_number_burn_cards,
    validate_number_player_cards::validate_number_player_cards,
    validate_number_total_cards::validate_number_total_cards,
    validate_player_ids::validate_player_ids,
};

use crate::types::Hand;

pub fn validate(hand: &Hand) -> Result<(), String> {
    // NOTE: order here good?
    validate_player_ids(&hand)?;
    validate_number_total_cards(&hand)?;
    validate_no_repeated_cards(&hand)?;
    validate_number_player_cards(&hand)?;
    validate_number_board_cards(&hand)?;
    validate_number_burn_cards(&hand)?;
    // validate_number_discarded_cards(&hand)?;
    // validate_number_deck_cards(&hand)?;

    Ok(())
}
