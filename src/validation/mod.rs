mod validate_optional_hand_properties;

use validate_optional_hand_properties::validate_optional_hand_properties;

use crate::types::Hand;

pub fn validate(hand: &Hand) -> Result<(), String> {
    validate_optional_hand_properties(&hand)?;
    // validate_player_ids(&hand)?; // check at least one player
    // validate_number_total_cards(&hand)?;
    // validate_no_repeated_cards(&hand)?;
    // validate_number_player_cards(&hand)?;
    // validate_number_board_cards(&hand)?;
    // validate_number_burn_cards(&hand)?;
    // validate_remainder_cards(&hand)?;

    Ok(())
}
