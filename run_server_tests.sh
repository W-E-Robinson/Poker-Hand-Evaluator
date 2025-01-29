#!/bin/bash

echo "Running server integration tests"

echo "Building Docker image"
docker build --pull --no-cache -t poker_hand_evaluator .
echo "Running container"
docker run --rm -d -p 8080:8080 --name poker_hand_evaluator poker_hand_evaluator

cargo nextest run \
  --test 404 \
  --test variants \
  --test general_validations \
  --test five_card_draw \
  --no-fail-fast

echo "Stopping container"
docker stop poker_hand_evaluator
