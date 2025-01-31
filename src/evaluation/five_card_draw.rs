// use crate::{EvaluationResponse, PlayerRequest, SuccessEvaluationResponse};

use crate::PlayerRequest;

// pub fn evaluate(players: &Vec<PlayerRequest>) -> Result<EvaluationResponse, String> {
pub fn evaluate(players: &Vec<PlayerRequest>) -> Result<(), String> {
    // let res = SuccessEvaluationResponse { // NOTE: or failure
    //     variant: String::from("five-card-draw"),
    //     players: vec![],
    // };

    return Err(String::from("placeholder error"));
}
