mod evaluate;

enum Suit {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

struct Card {
    suit: Suit,
    rank: Rank,
}

// Not all poker variants have a board and players' cards tuples handled in sub modules
struct BasePlayerEvaluation {
    display: String,
    hand: String,
    winner: bool,
}
