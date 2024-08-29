# Poker Hand Evaluator
# Not yet completed

This repo contains a package and accompanying Dockerfile web server for evaluating poker hands.

## Poker Variants
- Five card draw

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
docker run --rm -d -p 8080:8080 --name poker_hand_evaluator poker_hand_evaluator
```

2. Stop container:
```sh
docker stop poker_hand_evaluator
```

## API

### Get all supported poker variants
```http
  GET /variants
```

#### Example response:
```json
{
    "message": "List of supported poker variants to evaluate.",
    "variants": [
        {
            "pathParameter": "five-card-draw",
            "display": {
                "default": "Five-card draw",
                "alternates": ["Cantredraw"]
            }
        }
    ]
}
```

### Evaluate hands

```http
  POST /evaluate/five-card-draw
```

#### Example request body:
```json
{
    "players": [
        {
            "display": "player 1",
            "cards": ["Ah", "Kh", "Th", "Jh", "Qh"]
        },
        {
            "display": "player 2",
            "cards": ["2d", "3d", "4d", "5d", "6d"]
        },
    ]
}
```

#### Example response:
```json
{
    "players": [
        {
            "display": "player 1",
            "cards": ["Ah", "Kh", "Th", "Jh", "Qh"],
            "hand": "Ace high straight flush",
            "winner": true
        },
        {
            "display": "player 2",
            "cards": ["2d", "3d", "4d", "5d", "6d"],
            "hand": "Six high straight flush",
            "winner": false
        },
    ]
}
```
