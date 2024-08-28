# Poker Hand Evaluator

## Docker Commands
docker build -t poker_hand_evaluator .
docker run --rm -d -p 8080:8080 --name poker_hand_evaluator poker_hand_evaluator
docker stop poker_hand_evaluator
