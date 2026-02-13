use crate::{Card, Rank};

pub enum HandRank<'a> {
    HighCard {
        high_card: &'a Card,
        first_kicker: &'a Card,
        second_kicker: &'a Card,
        third_kicker: &'a Card,
        fourth_kicker: &'a Card,
    },
    Pair {
        pair: [&'a Card; 2],
        first_kicker: &'a Card,
        second_kicker: &'a Card,
        third_kicker: &'a Card,
    },
    TwoPair {
        high_pair: [&'a Card; 2],
        low_pair: [&'a Card; 2],
        kicker: &'a Card,
    },
    ThreeOfAKind {
        trips: [&'a Card; 3],
        first_kicker: &'a Card,
        second_kicker: &'a Card,
    },
    Straight {
        high_card: &'a Card,
        low_card: &'a Card,
    },
    Flush {
        first_card: &'a Card,
        second_card: &'a Card,
        third_card: &'a Card,
        fourth_card: &'a Card,
        fifth_card: &'a Card,
    },
    FullHouse {
        trips: [&'a Card; 3],
        pair: [&'a Card; 2],
    },
    FourOfAKind {
        quads: [&'a Card; 4],
        kicker: &'a Card,
    },
    StraightFlush {
        high_card: &'a Card,
        low_card: &'a Card,
    },
}

// NOTE: maybe this goes into a hand rank mod or seomthing? not just display now
impl<'a> HandRank<'a> {
    pub fn rank_number(&self) -> usize {
        match self {
            HandRank::HighCard {
                high_card: _,
                first_kicker: _,
                second_kicker: _,
                third_kicker: _,
                fourth_kicker: _,
            } => 0,
            HandRank::Pair {
                pair: _,
                first_kicker: _,
                second_kicker: _,
                third_kicker: _,
            } => 1,
            HandRank::TwoPair {
                high_pair: _,
                low_pair: _,
                kicker: _,
            } => 2,
            HandRank::ThreeOfAKind {
                trips: _,
                first_kicker: _,
                second_kicker: _,
            } => 3,
            HandRank::Straight {
                high_card: _,
                low_card: _,
            } => 4,
            HandRank::Flush {
                first_card: _,
                second_card: _,
                third_card: _,
                fourth_card: _,
                fifth_card: _,
            } => 5,
            HandRank::FullHouse { trips: _, pair: _ } => 6,
            HandRank::FourOfAKind {
                quads: _,
                kicker: _,
            } => 7,
            HandRank::StraightFlush {
                high_card: _,
                low_card: _,
            } => 8,
        }
    }

    fn to_display(&self) -> String {
        match self {
            HandRank::HighCard {
                high_card,
                first_kicker,
                second_kicker,
                third_kicker,
                fourth_kicker,
            } => {
                format!(
                    "High card {} with {}, {}, {}, and {} kickers",
                    high_card.to_rank_string(false),
                    first_kicker.to_rank_string(false),
                    second_kicker.to_rank_string(false),
                    third_kicker.to_rank_string(false),
                    fourth_kicker.to_rank_string(false),
                )
            }
            HandRank::Pair {
                pair,
                first_kicker,
                second_kicker,
                third_kicker,
            } => {
                format!(
                    "Pair of {} with {}, {}, and {} kickers",
                    pair[0].to_rank_string(true),
                    first_kicker.to_rank_string(false),
                    second_kicker.to_rank_string(false),
                    third_kicker.to_rank_string(false),
                )
            }
            HandRank::TwoPair {
                high_pair,
                low_pair,
                kicker,
            } => {
                format!(
                    "Two pair, {} over {} with {} kicker",
                    high_pair[0].to_rank_string(true),
                    low_pair[0].to_rank_string(true),
                    kicker.to_rank_string(false),
                )
            }
            HandRank::ThreeOfAKind {
                trips,
                first_kicker,
                second_kicker,
            } => {
                format!(
                    "Three of a kind, {} and {}, {} kickers",
                    trips[0].to_rank_string(true),
                    first_kicker.to_rank_string(false),
                    second_kicker.to_rank_string(false),
                )
            }
            HandRank::Straight {
                high_card,
                low_card,
            } => {
                let straight;
                let is_high_card_ace = high_card.rank == Rank::Ace;
                let is_low_card_ten = low_card.rank == Rank::Ten;
                let is_low_card_two = low_card.rank == Rank::Two;

                if is_high_card_ace && is_low_card_ten {
                    straight = String::from("Broadway");
                } else if is_high_card_ace && is_low_card_two {
                    straight = String::from("Wheel");
                } else {
                    straight = format!("{} high straight", high_card.to_rank_string(false));
                }

                format!("{}", straight,)
            }
            HandRank::Flush {
                first_card,
                second_card,
                third_card,
                fourth_card,
                fifth_card,
            } => {
                format!(
                    "{} high flush, with {}, {}, {}, {}",
                    first_card.to_rank_string(false),
                    second_card.to_rank_string(false),
                    third_card.to_rank_string(false),
                    fourth_card.to_rank_string(false),
                    fifth_card.to_rank_string(false),
                )
            }
            HandRank::FullHouse { trips, pair } => format!(
                "Full house, {} full of {}",
                trips[0].to_rank_string(true),
                pair[0].to_rank_string(true),
            ),
            HandRank::FourOfAKind { quads, kicker } => {
                format!(
                    "Four of a kind, {} with {} kicker",
                    quads[0].to_rank_string(true),
                    kicker.to_rank_string(false),
                )
            }
            HandRank::StraightFlush {
                high_card,
                low_card,
            } => {
                let straight_flush;
                let is_high_card_ace = high_card.rank == Rank::Ace;
                let is_low_card_ten = low_card.rank == Rank::Ten;
                let is_low_card_two = low_card.rank == Rank::Two;

                if is_high_card_ace && is_low_card_ten {
                    straight_flush = String::from("Royal flush");
                } else if is_high_card_ace && is_low_card_two {
                    straight_flush = String::from("Wheel flush");
                } else {
                    straight_flush =
                        format!("{} high straight flush", high_card.to_rank_string(false));
                }

                format!("{}", straight_flush)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Card, Rank, Suit};

    use super::*;

    #[test]
    fn test_high_card_to_display() {
        let cards = HandRank::HighCard {
            high_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            }),
            first_kicker: (&Card {
                suit: Suit::Spade,
                rank: Rank::King,
            }),
            second_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            }),
            third_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Three,
            }),
            fourth_kicker: (&Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "High card Ace with King, Jack, Three, and Two kickers"
        );
    }

    #[test]
    fn test_pair_to_display() {
        let cards = HandRank::Pair {
            pair: ([
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Two,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Two,
                },
            ]),
            first_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            }),
            second_kicker: (&Card {
                suit: Suit::Spade,
                rank: Rank::King,
            }),
            third_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "Pair of Twos with Ace, King, and Jack kickers"
        );
    }

    #[test]
    fn test_two_pair_to_display() {
        let cards = HandRank::TwoPair {
            high_pair: ([
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::King,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::King,
                },
            ]),
            low_pair: ([
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Six,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Six,
                },
            ]),
            kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "Two pair, Kings over Sixes with Jack kicker",
        );
    }

    #[test]
    fn test_three_of_a_kind_to_display() {
        let cards = HandRank::ThreeOfAKind {
            trips: ([
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Two,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Two,
                },
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::Two,
                },
            ]),
            first_kicker: (&Card {
                suit: Suit::Heart,
                rank: Rank::King,
            }),
            second_kicker: (&Card {
                suit: Suit::Club,
                rank: Rank::Jack,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "Three of a kind, Twos and King, Jack kickers",
        );
    }

    #[test]
    fn test_straight_to_display() {
        let cards = HandRank::Straight {
            high_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Six,
            }),
            low_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Two,
            }),
        };
        assert_eq!(cards.to_display(), "Six high straight",);
    }

    #[test]
    fn test_wheel_straight_to_display() {
        let cards = HandRank::Straight {
            high_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            }),
            low_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Two,
            }),
        };
        assert_eq!(cards.to_display(), "Wheel",);
    }

    #[test]
    fn test_broadway_straight_to_display() {
        let cards = HandRank::Straight {
            high_card: (&Card {
                suit: Suit::Club,
                rank: Rank::Ace,
            }),
            low_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Ten,
            }),
        };
        assert_eq!(cards.to_display(), "Broadway",);
    }

    #[test]
    fn test_flush_to_display() {
        let cards = HandRank::Flush {
            first_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::King,
            }),
            second_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Jack,
            }),
            third_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Ten,
            }),
            fourth_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Seven,
            }),
            fifth_card: (&Card {
                suit: Suit::Heart,
                rank: Rank::Six,
            }),
        };
        assert_eq!(
            cards.to_display(),
            "King high flush, with Jack, Ten, Seven, Six"
        );
    }

    #[test]
    fn test_full_house_to_display() {
        let cards = HandRank::FullHouse {
            trips: ([
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::Seven,
                },
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Seven,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Ten,
                },
            ]),
            pair: ([
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::Ten,
                },
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Ten,
                },
            ]),
        };
        assert_eq!(cards.to_display(), "Full house, Sevens full of Tens");
    }

    #[test]
    fn test_four_of_a_kind_house_to_display() {
        let cards = HandRank::FourOfAKind {
            quads: ([
                &Card {
                    suit: Suit::Diamond,
                    rank: Rank::Ten,
                },
                &Card {
                    suit: Suit::Spade,
                    rank: Rank::Ten,
                },
                &Card {
                    suit: Suit::Heart,
                    rank: Rank::Ten,
                },
                &Card {
                    suit: Suit::Club,
                    rank: Rank::Ten,
                },
            ]),
            kicker: (&Card {
                suit: Suit::Spade,
                rank: Rank::Seven,
            }),
        };
        assert_eq!(cards.to_display(), "Four of a kind, Tens with Seven kicker");
    }

    #[test]
    fn test_straight_flush_to_display() {
        let cards = HandRank::StraightFlush {
            high_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::King,
            }),
            low_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Nine,
            }),
        };
        assert_eq!(cards.to_display(), "King high straight flush");
    }

    #[test]
    fn test_wheel_flush_to_display() {
        let cards = HandRank::StraightFlush {
            high_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Ace,
            }),
            low_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Two,
            }),
        };
        assert_eq!(cards.to_display(), "Wheel flush");
    }

    #[test]
    fn test_royal_flush_to_display() {
        let cards = HandRank::StraightFlush {
            high_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Ace,
            }),
            low_card: (&Card {
                suit: Suit::Diamond,
                rank: Rank::Ten,
            }),
        };
        assert_eq!(cards.to_display(), "Royal flush");
    }
}
