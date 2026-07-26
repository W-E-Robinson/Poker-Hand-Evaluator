use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use poker_hand_evaluator::{
    evaluate_hand,
    types::{Card, ErrorType, Evaluation, Hand, Player, Rank, Suit, Variant},
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!("{}=debug,tower_http=debug", env!("CARGO_CRATE_NAME")).into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app()).await.unwrap();
}

fn app() -> Router {
    Router::new()
        .route("/variants", get(get_variants))
        .route("/evaluate", post(post_evaluate))
        .layer(TraceLayer::new_for_http())
}

#[derive(Serialize)]
struct VariantDisplayInfo {
    default: String,
    alternates: Vec<String>,
}
#[derive(Serialize)]
struct VariantInfo {
    http_request: String,
    display: VariantDisplayInfo,
}
async fn get_variants() -> Json<Vec<VariantInfo>> {
    let variants_response = vec![
        VariantInfo {
            http_request: String::from("POST /evaluate (body.variant=five-card-draw)"),
            display: VariantDisplayInfo {
                default: String::from("Five-card draw"),
                alternates: vec![String::from("Cantredraw")],
            },
        },
        VariantInfo {
            http_request: String::from("POST /evaluate (body.variant=texas-hold-em)"),
            display: VariantDisplayInfo {
                default: String::from("Texas Hold 'em"),
                alternates: vec![
                    String::from("Texas holdem"),
                    String::from("hold 'em"),
                    String::from("holdem"),
                ],
            },
        },
        VariantInfo {
            http_request: String::from("POST /evaluate (body.variant=four-card-omaha-hi)"),
            display: VariantDisplayInfo {
                default: String::from("Omaha Hold 'em"),
                alternates: vec![
                    String::from("Omaha holdem"),
                    String::from("Omaha"),
                    String::from("Four Card Omaha"),
                ],
            },
        },
    ];

    Json(variants_response)
}

#[derive(Deserialize, Serialize)]
pub struct EvaluateRequestPlayer {
    pub id: String,
    pub cards: Vec<String>,
}
#[derive(Deserialize, Serialize)]
pub struct EvaluateRequest {
    pub id: String,
    pub variant: String,
    pub players: Vec<EvaluateRequestPlayer>,
    pub board: Option<Vec<String>>,
    pub burn_cards: Option<Vec<String>>,
    pub discarded_cards: Option<Vec<String>>,
    pub remaining_deck: Vec<String>,
}
impl EvaluateRequest {
    pub fn into_domain(self) -> Result<Hand, String> {
        let variant = match self.variant.as_str() {
            "five-card-draw" => Variant::FiveCardDraw,
            "texas-hold-em" => Variant::TexasHoldem,
            "four-card-omaha-hi" => Variant::FourCardOmahaHi,
            _ => return Err(String::from("Unsupported variant provided.")),
        };

        let players = self
            .players
            .into_iter()
            .map(parse_player)
            .collect::<Result<Vec<_>, _>>()?;

        let board = self
            .board
            .map(|cards| {
                cards
                    .into_iter()
                    .map(parse_card)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        let burn_cards = self
            .burn_cards
            .map(|cards| {
                cards
                    .into_iter()
                    .map(parse_card)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        let discarded_cards = self
            .discarded_cards
            .map(|cards| {
                cards
                    .into_iter()
                    .map(parse_card)
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;

        let remaining_deck = self
            .remaining_deck
            .into_iter()
            .map(parse_card)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Hand {
            id: self.id,
            variant: variant,
            players: players,
            board: board,
            burn_cards: burn_cards,
            discarded_cards: discarded_cards,
            remaining_deck: remaining_deck,
        })
    }
}
fn parse_player(player: EvaluateRequestPlayer) -> Result<Player, String> {
    let cards = player
        .cards
        .into_iter()
        .map(parse_card)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Player {
        id: player.id,
        cards: cards,
    })
}
fn parse_card(string: String) -> Result<Card, String> {
    if string.len() != 2 {
        return Err(String::from("Incorrect card format."));
    }

    let (rank_str, suit_str) = string.split_at(string.len() - 1);

    let rank = match rank_str {
        "A" => Rank::Ace,
        "K" => Rank::King,
        "Q" => Rank::Queen,
        "J" => Rank::Jack,
        "T" => Rank::Ten,
        "9" => Rank::Nine,
        "8" => Rank::Eight,
        "7" => Rank::Seven,
        "6" => Rank::Six,
        "5" => Rank::Five,
        "4" => Rank::Four,
        "3" => Rank::Three,
        "2" => Rank::Two,
        _ => return Err(format!("Unrecognised rank: {}.", rank_str)),
    };

    let suit = match suit_str {
        "h" => Suit::Heart,
        "d" => Suit::Diamond,
        "c" => Suit::Club,
        "s" => Suit::Spade,
        _ => return Err(format!("Unrecognised suit: {}.", suit_str)),
    };

    Ok(Card { rank, suit })
}
async fn post_evaluate(
    Json(body): Json<EvaluateRequest>,
) -> Result<Json<Evaluation>, (StatusCode, String)> {
    let hand = body
        .into_domain()
        .map_err(|e| (StatusCode::BAD_REQUEST, e))?;

    let evaluation = match evaluate_hand(hand) {
        Ok(eval) => eval,
        Err(error) => {
            if error.error_type == ErrorType::Validation {
                return Err((StatusCode::BAD_REQUEST, error.message));
            } else {
                return Err((StatusCode::INTERNAL_SERVER_ERROR, error.message));
            }
        }
    };
    Ok(Json(evaluation))
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{header::CONTENT_TYPE, Request, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::{json, Value};
    use tower::ServiceExt;

    #[test]
    fn test_parse_card_unrecognised_rank() {
        let result = parse_card(String::from("Xh")).unwrap_err();
        assert_eq!(result, "Unrecognised rank: X.");
    }

    #[test]
    fn test_parse_card_unrecognised_suit() {
        let result = parse_card(String::from("At")).unwrap_err();
        assert_eq!(result, "Unrecognised suit: t.");
    }

    #[test]
    fn test_parse_card_incorrect_string_format() {
        let result = parse_card(String::from("")).unwrap_err();
        assert_eq!(result, "Incorrect card format.");
    }

    #[test]
    fn test_parse_card_ace_heart() {
        assert_eq!(
            parse_card(String::from("Ah")),
            Ok(Card {
                rank: Rank::Ace,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_king_heart() {
        assert_eq!(
            parse_card(String::from("Kh")),
            Ok(Card {
                rank: Rank::King,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_queen_heart() {
        assert_eq!(
            parse_card(String::from("Qh")),
            Ok(Card {
                rank: Rank::Queen,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_jack_heart() {
        assert_eq!(
            parse_card(String::from("Jh")),
            Ok(Card {
                rank: Rank::Jack,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_ten_heart() {
        assert_eq!(
            parse_card(String::from("Th")),
            Ok(Card {
                rank: Rank::Ten,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_nine_heart() {
        assert_eq!(
            parse_card(String::from("9h")),
            Ok(Card {
                rank: Rank::Nine,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_eight_heart() {
        assert_eq!(
            parse_card(String::from("8h")),
            Ok(Card {
                rank: Rank::Eight,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_seven_heart() {
        assert_eq!(
            parse_card(String::from("7h")),
            Ok(Card {
                rank: Rank::Seven,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_six_heart() {
        assert_eq!(
            parse_card(String::from("6h")),
            Ok(Card {
                rank: Rank::Six,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_five_heart() {
        assert_eq!(
            parse_card(String::from("5h")),
            Ok(Card {
                rank: Rank::Five,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_four_heart() {
        assert_eq!(
            parse_card(String::from("4h")),
            Ok(Card {
                rank: Rank::Four,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_three_heart() {
        assert_eq!(
            parse_card(String::from("3h")),
            Ok(Card {
                rank: Rank::Three,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_two_heart() {
        assert_eq!(
            parse_card(String::from("2h")),
            Ok(Card {
                rank: Rank::Two,
                suit: Suit::Heart,
            })
        );
    }

    #[test]
    fn test_parse_card_ace_club() {
        assert_eq!(
            parse_card(String::from("Ac")),
            Ok(Card {
                rank: Rank::Ace,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_king_club() {
        assert_eq!(
            parse_card(String::from("Kc")),
            Ok(Card {
                rank: Rank::King,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_queen_club() {
        assert_eq!(
            parse_card(String::from("Qc")),
            Ok(Card {
                rank: Rank::Queen,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_jack_club() {
        assert_eq!(
            parse_card(String::from("Jc")),
            Ok(Card {
                rank: Rank::Jack,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_ten_club() {
        assert_eq!(
            parse_card(String::from("Tc")),
            Ok(Card {
                rank: Rank::Ten,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_nine_club() {
        assert_eq!(
            parse_card(String::from("9c")),
            Ok(Card {
                rank: Rank::Nine,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_eight_club() {
        assert_eq!(
            parse_card(String::from("8c")),
            Ok(Card {
                rank: Rank::Eight,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_seven_club() {
        assert_eq!(
            parse_card(String::from("7c")),
            Ok(Card {
                rank: Rank::Seven,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_six_club() {
        assert_eq!(
            parse_card(String::from("6c")),
            Ok(Card {
                rank: Rank::Six,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_five_club() {
        assert_eq!(
            parse_card(String::from("5c")),
            Ok(Card {
                rank: Rank::Five,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_four_club() {
        assert_eq!(
            parse_card(String::from("4c")),
            Ok(Card {
                rank: Rank::Four,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_three_club() {
        assert_eq!(
            parse_card(String::from("3c")),
            Ok(Card {
                rank: Rank::Three,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_two_club() {
        assert_eq!(
            parse_card(String::from("2c")),
            Ok(Card {
                rank: Rank::Two,
                suit: Suit::Club,
            })
        );
    }

    #[test]
    fn test_parse_card_ace_diamond() {
        assert_eq!(
            parse_card(String::from("Ad")),
            Ok(Card {
                rank: Rank::Ace,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_king_diamond() {
        assert_eq!(
            parse_card(String::from("Kd")),
            Ok(Card {
                rank: Rank::King,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_queen_diamond() {
        assert_eq!(
            parse_card(String::from("Qd")),
            Ok(Card {
                rank: Rank::Queen,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_jack_diamond() {
        assert_eq!(
            parse_card(String::from("Jd")),
            Ok(Card {
                rank: Rank::Jack,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_ten_diamond() {
        assert_eq!(
            parse_card(String::from("Td")),
            Ok(Card {
                rank: Rank::Ten,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_nine_diamond() {
        assert_eq!(
            parse_card(String::from("9d")),
            Ok(Card {
                rank: Rank::Nine,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_eight_diamond() {
        assert_eq!(
            parse_card(String::from("8d")),
            Ok(Card {
                rank: Rank::Eight,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_seven_diamond() {
        assert_eq!(
            parse_card(String::from("7d")),
            Ok(Card {
                rank: Rank::Seven,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_six_diamond() {
        assert_eq!(
            parse_card(String::from("6d")),
            Ok(Card {
                rank: Rank::Six,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_five_diamond() {
        assert_eq!(
            parse_card(String::from("5d")),
            Ok(Card {
                rank: Rank::Five,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_four_diamond() {
        assert_eq!(
            parse_card(String::from("4d")),
            Ok(Card {
                rank: Rank::Four,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_three_diamond() {
        assert_eq!(
            parse_card(String::from("3d")),
            Ok(Card {
                rank: Rank::Three,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_two_diamond() {
        assert_eq!(
            parse_card(String::from("2d")),
            Ok(Card {
                rank: Rank::Two,
                suit: Suit::Diamond,
            })
        );
    }

    #[test]
    fn test_parse_card_ace_spade() {
        assert_eq!(
            parse_card(String::from("As")),
            Ok(Card {
                rank: Rank::Ace,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_king_spade() {
        assert_eq!(
            parse_card(String::from("Ks")),
            Ok(Card {
                rank: Rank::King,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_queen_spade() {
        assert_eq!(
            parse_card(String::from("Qs")),
            Ok(Card {
                rank: Rank::Queen,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_jack_spade() {
        assert_eq!(
            parse_card(String::from("Js")),
            Ok(Card {
                rank: Rank::Jack,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_ten_spade() {
        assert_eq!(
            parse_card(String::from("Ts")),
            Ok(Card {
                rank: Rank::Ten,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_nine_spade() {
        assert_eq!(
            parse_card(String::from("9s")),
            Ok(Card {
                rank: Rank::Nine,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_eight_spade() {
        assert_eq!(
            parse_card(String::from("8s")),
            Ok(Card {
                rank: Rank::Eight,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_seven_spade() {
        assert_eq!(
            parse_card(String::from("7s")),
            Ok(Card {
                rank: Rank::Seven,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_six_spade() {
        assert_eq!(
            parse_card(String::from("6s")),
            Ok(Card {
                rank: Rank::Six,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_five_spade() {
        assert_eq!(
            parse_card(String::from("5s")),
            Ok(Card {
                rank: Rank::Five,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_four_spade() {
        assert_eq!(
            parse_card(String::from("4s")),
            Ok(Card {
                rank: Rank::Four,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_three_spade() {
        assert_eq!(
            parse_card(String::from("3s")),
            Ok(Card {
                rank: Rank::Three,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_card_two_spade() {
        assert_eq!(
            parse_card(String::from("2s")),
            Ok(Card {
                rank: Rank::Two,
                suit: Suit::Spade,
            })
        );
    }

    #[test]
    fn test_parse_player_error_parsing_player_cards() {
        let result = parse_player(EvaluateRequestPlayer {
            id: String::from("player-1-id"),
            cards: vec![
                String::from("Ax"),
                String::from("Ac"),
                String::from("Ad"),
                String::from("As"),
                String::from("Ks"),
            ],
        })
        .unwrap_err();

        assert_eq!(result, "Unrecognised suit: x.");
    }

    #[test]
    fn test_parse_player() {
        assert_eq!(
            parse_player(EvaluateRequestPlayer {
                id: String::from("player-1-id"),
                cards: vec![
                    String::from("Kc"),
                    String::from("Ac"),
                    String::from("Ad"),
                    String::from("As"),
                    String::from("Ks"),
                ],
            }),
            Ok(Player {
                id: String::from("player-1-id"),
                cards: vec![
                    Card {
                        rank: Rank::King,
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Club,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Diamond,
                    },
                    Card {
                        rank: Rank::Ace,
                        suit: Suit::Spade,
                    },
                    Card {
                        rank: Rank::King,
                        suit: Suit::Spade,
                    },
                ]
            })
        );
    }

    #[test]
    fn test_evaluate_request_impl_into_domain_error_players() {
        let request = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("five-card-draw"),
            players: vec![EvaluateRequestPlayer {
                id: String::from("player-1-id"),
                cards: vec![String::from("Xc")],
            }],
            remaining_deck: vec![String::from("Ac")],
            board: None,
            burn_cards: None,
            discarded_cards: None,
        };
        let result = request.into_domain().unwrap_err();

        assert_eq!(result, "Unrecognised rank: X.");
    }

    #[test]
    fn test_evaluate_request_impl_into_domain_error_board() {
        let request = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("five-card-draw"),
            players: vec![EvaluateRequestPlayer {
                id: String::from("player-1-id"),
                cards: vec![String::from("Ac")],
            }],
            remaining_deck: vec![String::from("Ac")],
            board: Some(vec![String::from("Xc")]),
            burn_cards: None,
            discarded_cards: None,
        };
        let result = request.into_domain().unwrap_err();

        assert_eq!(result, "Unrecognised rank: X.");
    }

    #[test]
    fn test_evaluate_request_impl_into_domain_error_burn_cards() {
        let request = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("five-card-draw"),
            players: vec![EvaluateRequestPlayer {
                id: String::from("player-1-id"),
                cards: vec![String::from("Ac")],
            }],
            remaining_deck: vec![String::from("Ac")],
            board: None,
            burn_cards: Some(vec![String::from("Xc")]),
            discarded_cards: None,
        };
        let result = request.into_domain().unwrap_err();

        assert_eq!(result, "Unrecognised rank: X.");
    }

    #[test]
    fn test_evaluate_request_impl_into_domain_error_disacarded_cards() {
        let request = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("five-card-draw"),
            players: vec![EvaluateRequestPlayer {
                id: String::from("player-1-id"),
                cards: vec![String::from("Ac")],
            }],
            remaining_deck: vec![String::from("Ac")],
            board: None,
            burn_cards: None,
            discarded_cards: Some(vec![String::from("Xc")]),
        };
        let result = request.into_domain().unwrap_err();

        assert_eq!(result, "Unrecognised rank: X.");
    }

    #[test]
    fn test_evaluate_request_impl_into_domain_valid() {
        let request = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("five-card-draw"),
            players: vec![EvaluateRequestPlayer {
                id: String::from("player-1-id"),
                cards: vec![String::from("Ac")],
            }],
            remaining_deck: vec![String::from("Ac")],
            board: None,
            burn_cards: None,
            discarded_cards: None,
        };
        let result = request.into_domain();

        assert_eq!(
            result,
            Ok(Hand {
                id: String::from("hand-id"),
                variant: Variant::FiveCardDraw,
                players: vec![Player {
                    id: String::from("player-1-id"),
                    cards: vec![Card {
                        rank: Rank::Ace,
                        suit: Suit::Club,
                    }],
                }],
                remaining_deck: vec![Card {
                    rank: Rank::Ace,
                    suit: Suit::Club,
                }],
                board: None,
                burn_cards: None,
                discarded_cards: None,
            })
        );
    }

    #[tokio::test]
    async fn test_not_found() {
        let app = app();

        let response = app
            .oneshot(Request::get("/does-not-exist").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert!(body.is_empty());
    }

    #[tokio::test]
    async fn test_get_variants() {
        let app = app();

        let response = app
            .oneshot(Request::get("/variants").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let body: Value = serde_json::from_slice(&body).unwrap();

        let expected_response = vec![
            VariantInfo {
                http_request: String::from("POST /evaluate (body.variant=five-card-draw)"),
                display: VariantDisplayInfo {
                    default: String::from("Five-card draw"),
                    alternates: vec![String::from("Cantredraw")],
                },
            },
            VariantInfo {
                http_request: String::from("POST /evaluate (body.variant=texas-hold-em)"),
                display: VariantDisplayInfo {
                    default: String::from("Texas Hold 'em"),
                    alternates: vec![
                        String::from("Texas holdem"),
                        String::from("hold 'em"),
                        String::from("holdem"),
                    ],
                },
            },
            VariantInfo {
                http_request: String::from("POST /evaluate (body.variant=four-card-omaha-hi)"),
                display: VariantDisplayInfo {
                    default: String::from("Omaha Hold 'em"),
                    alternates: vec![
                        String::from("Omaha holdem"),
                        String::from("Omaha"),
                        String::from("Four Card Omaha"),
                    ],
                },
            },
        ];
        assert_eq!(body, serde_json::to_value(&expected_response).unwrap());
    }

    #[tokio::test]
    async fn test_post_evaluate_invalid_request_missing_properties() {
        let app = app();

        let response = app
            .oneshot(
                Request::post("/evaluate")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(r#"{"hello":"world"}"#))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn test_unsupported_variant_error() {
        let app = app();

        let body = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("not-a-variant"),
            players: vec![EvaluateRequestPlayer {
                id: String::from("player-1-id"),
                cards: vec![String::from("Ac")],
            }],
            remaining_deck: vec![String::from("Ac")],
            board: None,
            burn_cards: None,
            discarded_cards: None,
        };
        let json = serde_json::to_string(&body).unwrap();

        let response = app
            .oneshot(
                Request::post("/evaluate")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Unsupported variant provided.");
    }

    #[tokio::test]
    async fn test_evaluation_error() {
        let app = app();

        let body = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("five-card-draw"),
            players: vec![EvaluateRequestPlayer {
                id: String::from("player-1-id"),
                cards: vec![String::from("Ac")],
            }],
            remaining_deck: vec![String::from("Ac")],
            board: None,
            burn_cards: None,
            discarded_cards: None,
        };
        let json = serde_json::to_string(&body).unwrap();

        let response = app
            .oneshot(
                Request::post("/evaluate")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Exactly 52 cards must be provided in total.");
    }

    #[tokio::test]
    async fn test_evaluation_success_five_card_draw() {
        let app = app();

        let body = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("five-card-draw"),
            players: vec![
                EvaluateRequestPlayer {
                    id: String::from("player-1-id"),
                    cards: vec![
                        String::from("Ac"),
                        String::from("Kc"),
                        String::from("Qc"),
                        String::from("Jc"),
                        String::from("Tc"),
                    ],
                },
                EvaluateRequestPlayer {
                    id: String::from("player-2-id"),
                    cards: vec![
                        String::from("Ad"),
                        String::from("Kd"),
                        String::from("Qd"),
                        String::from("Jd"),
                        String::from("Td"),
                    ],
                },
            ],
            remaining_deck: vec![
                String::from("9c"),
                String::from("8c"),
                String::from("7c"),
                String::from("6c"),
                String::from("5c"),
                String::from("4c"),
                String::from("3c"),
                String::from("2c"),
                String::from("9d"),
                String::from("8d"),
                String::from("7d"),
                String::from("6d"),
                String::from("5d"),
                String::from("4d"),
                String::from("3d"),
                String::from("2d"),
                String::from("As"),
                String::from("Ks"),
                String::from("Qs"),
                String::from("Js"),
                String::from("Ts"),
                String::from("9s"),
                String::from("8s"),
                String::from("7s"),
                String::from("6s"),
                String::from("5s"),
                String::from("4s"),
                String::from("3s"),
                String::from("2s"),
                String::from("Ah"),
                String::from("Kh"),
                String::from("Qh"),
                String::from("Jh"),
                String::from("Th"),
                String::from("9h"),
                String::from("8h"),
                String::from("7h"),
                String::from("6h"),
                String::from("5h"),
                String::from("4h"),
                String::from("3h"),
                String::from("2h"),
            ],
            board: None,
            burn_cards: None,
            discarded_cards: Some(vec![]),
        };
        let json = serde_json::to_string(&body).unwrap();

        let response = app
            .oneshot(
                Request::post("/evaluate")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let expected = json!({
            "id": "hand-id",
            "players": [
                { "id": "player-1-id", "hand": "Royal Flush" },
                { "id": "player-2-id", "hand": "Royal Flush" }
            ],
            "winners": ["player-1-id", "player-2-id"],
            "winning_hand": "Royal Flush"
        });

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_evaluation_success_texas_hold_em() {
        let app = app();

        let body = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("texas-hold-em"),
            players: vec![
                EvaluateRequestPlayer {
                    id: String::from("player-1-id"),
                    cards: vec![String::from("Ac"), String::from("Kc")],
                },
                EvaluateRequestPlayer {
                    id: String::from("player-2-id"),
                    cards: vec![String::from("Jd"), String::from("Td")],
                },
            ],
            board: Some(vec![
                String::from("Qc"),
                String::from("Jc"),
                String::from("Tc"),
                String::from("Qd"),
                String::from("7c"),
            ]),
            remaining_deck: vec![
                String::from("Ad"),
                String::from("Kd"),
                String::from("9c"),
                String::from("8c"),
                String::from("6c"),
                String::from("5c"),
                String::from("4c"),
                String::from("3c"),
                String::from("2c"),
                String::from("9d"),
                String::from("8d"),
                String::from("7d"),
                String::from("6d"),
                String::from("5d"),
                String::from("4d"),
                String::from("3d"),
                String::from("2d"),
                String::from("As"),
                String::from("Ks"),
                String::from("Qs"),
                String::from("Js"),
                String::from("Ts"),
                String::from("9s"),
                String::from("8s"),
                String::from("7s"),
                String::from("6s"),
                String::from("5s"),
                String::from("4s"),
                String::from("3s"),
                String::from("2s"),
                String::from("Ah"),
                String::from("Kh"),
                String::from("Qh"),
                String::from("Jh"),
                String::from("Th"),
                String::from("9h"),
                String::from("8h"),
                String::from("7h"),
                String::from("6h"),
                String::from("5h"),
            ],
            burn_cards: Some(vec![
                String::from("4h"),
                String::from("3h"),
                String::from("2h"),
            ]),
            discarded_cards: None,
        };
        let json = serde_json::to_string(&body).unwrap();

        let response = app
            .oneshot(
                Request::post("/evaluate")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let expected = json!({
            "id": "hand-id",
            "players": [
                { "id": "player-1-id", "hand": "Royal Flush" },
                { "id": "player-2-id", "hand": "Two Pair, Queens and Jacks" }
            ],
            "winners": ["player-1-id",],
            "winning_hand": "Royal Flush"
        });

        assert_eq!(actual, expected);
    }

    #[tokio::test]
    async fn test_evaluation_success_four_card_omaha_hi() {
        let app = app();

        let body = EvaluateRequest {
            id: String::from("hand-id"),
            variant: String::from("four-card-omaha-hi"),
            players: vec![
                EvaluateRequestPlayer {
                    id: String::from("player-1-id"),
                    cards: vec![
                        String::from("Ad"),
                        String::from("Kd"),
                        String::from("2c"),
                        String::from("7s"),
                    ],
                },
                EvaluateRequestPlayer {
                    id: String::from("player-2-id"),
                    cards: vec![
                        String::from("9h"),
                        String::from("9s"),
                        String::from("2d"),
                        String::from("6c"),
                    ],
                },
            ],
            board: Some(vec![
                String::from("Qd"),
                String::from("Jd"),
                String::from("Td"),
                String::from("3h"),
                String::from("4c"),
            ]),
            remaining_deck: vec![
                String::from("2h"),
                String::from("4h"),
                String::from("8h"),
                String::from("Th"),
                String::from("Jh"),
                String::from("Qh"),
                String::from("Kh"),
                String::from("Ah"),
                String::from("3d"),
                String::from("4d"),
                String::from("5d"),
                String::from("6d"),
                String::from("7d"),
                String::from("8d"),
                String::from("9d"),
                String::from("3c"),
                String::from("5c"),
                String::from("7c"),
                String::from("8c"),
                String::from("9c"),
                String::from("Tc"),
                String::from("Jc"),
                String::from("Qc"),
                String::from("Kc"),
                String::from("Ac"),
                String::from("2s"),
                String::from("3s"),
                String::from("4s"),
                String::from("5s"),
                String::from("6s"),
                String::from("8s"),
                String::from("Ts"),
                String::from("Js"),
                String::from("Qs"),
                String::from("Ks"),
                String::from("As"),
            ],
            burn_cards: Some(vec![
                String::from("5h"),
                String::from("6h"),
                String::from("7h"),
            ]),
            discarded_cards: None,
        };
        let json = serde_json::to_string(&body).unwrap();

        let response = app
            .oneshot(
                Request::post("/evaluate")
                    .header(CONTENT_TYPE, "application/json")
                    .body(Body::from(json))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        let actual: serde_json::Value = serde_json::from_slice(&body).unwrap();

        let expected = json!({
            "id": "hand-id",
            "players": [
                { "id": "player-1-id", "hand": "Royal Flush" },
                { "id": "player-2-id", "hand": "Pair of Nines" }
            ],
            "winners": ["player-1-id"],
            "winning_hand": "Royal Flush"
        });

        assert_eq!(actual, expected);
    }
}
