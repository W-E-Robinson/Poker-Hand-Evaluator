## Poker Hand Evaluator

### Contents
- [Intro](#intro)
- [Supported Variants](#supported-variants)
- [Library Code](#library-code)
- [Dockerised Server Binary](#dockerised-server-binary)
- [Testing](#testing)
- [Future Ideas](#future-ideas)

### Intro
This repo contains library code for evaluating poker hands. It also contains a dockerised image of a binary for a HTTP server that sits infront of the evaluation engine (this is ultimately what I needed).

### Supported Variants
```rust
pub enum Variant {
    FiveCardDraw,
}
```

### Library Code
The core evaluation logic is inspired by this [article](https://javascript.plainenglish.io/building-a-poker-hand-evaluator-without-conditional-branches-556c39c8e33e). Its idea to apply a bitmask encoding to the card ranks vastly simplifies the logic.
```rust
// Single exposed function
pub fn evaluate_hand(hand: Hand) -> Result<Evaluation, Error>
```

```rust
// Input
pub struct Hand {
    pub id: String,
    pub variant: Variant,
    pub players: Vec<Player>,
    pub board: Option<Vec<Card>>,
    pub burn_cards: Option<Vec<Card>>,
    pub discarded_cards: Option<Vec<Card>>,
    pub remaining_deck: Vec<Card>,
}
```

```rust
// Output
pub struct Evaluation {
    pub id: String,
    pub players: Vec<PlayerEval>,
    pub winners: Vec<String>,
    pub winning_hand: String,
}
pub struct PlayerEval {
    pub id: String,
    pub hand: String,
}
```

```rust
// Error
pub struct Error {
    pub id: String,
    pub error_type: ErrorType,
    pub message: String,
}
pub enum ErrorType {
    Validation,
}
```

### Dockerised Server Binary
#### Running server binary
```sh
cargo run --bin server
```

#### Get all supported poker variants
```http
curl localhost:8080/variants
```
```json
[
    {
        "http_request": "POST /evaluate?variant=five-card-draw",
        "display": {
            "default": "Five-card draw",
            "alternates": [
                "Cantredraw"
            ]
        }
    }
]
```

#### Evaluate hand
```http
curl \
  --header "Content-Type: application/json" \
  --request POST \
  --data '{
    "id":"hand-id",
    "variant":"five-card-draw",
    "players":[
      {
        "id":"player-1-id",
        "cards":["Ac","Kc","Qc","Jc","Tc"]
      },
      {
        "id":"player-2-id",
        "cards":["Ad","Kd","Qd","Jd","Td"]
      }
    ],
    "discarded_cards":[],
    "remaining_deck":[
      "9c","8c","7c","6c","5c","4c","3c","2c",
      "9d","8d","7d","6d","5d","4d","3d","2d",
      "As","Ks","Qs","Js","Ts","9s","8s","7s",
      "6s","5s","4s","3s","2s",
      "Ah","Kh","Qh","Jh","Th","9h","8h","7h",
      "6h","5h","4h","3h","2h"
    ]
  }' \
  "http://localhost:8080/evaluate?variant=five-card-draw"
```
```json
{
    "id": "hand-id",
    "players": [
        {
            "id": "player-1-id",
            "hand": "Royal Flush"
        },
        {
            "id": "player-2-id",
            "hand": "Royal Flush"
        }
    ],
    "winners": [
        "player-1-id",
        "player-2-id"
    ],
    "winning_hand": "Royal Flush"
}
```

### Testing
#### Library + Server tests
```sh
make test
```
#### Only Library tests
```sh
make test-lib
```
#### Only Server tests
```sh
make test-server
```

### Future Ideas / Improvements
- improved hand displays (e.g. Straight -> Six High Straight)
- new variant - texas holdem
- new variant - 4 card pot limit omaha hi
- new variant - 5 card pot limit omaha hi
- new variant - 6 card pot limit omaha hi

# STUFF BELOW IS ALL OLD README BITS = REMOVE

### Server API tests
```sh
chmod +x ./run_server_tests.sh
./run_server_tests.sh
```
## Usage
### Use locally installed Rust:
1. Compile and run:
```sh
cargo run
```

### Use Docker:
1. Build image:
```sh
docker build --pull --no-cache -t poker_hand_evaluator .
```
2. Run container:
```sh
docker run --rm -p 8080:8080 --name poker_hand_evaluator poker_hand_evaluator
```
3. Stop container:
```sh
docker stop poker_hand_evaluator
```


