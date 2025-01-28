#!/bin/bash

docker build --pull --no-cache -t poker_hand_evaluator .
docker run -d -p 8080:8080 --name poker_hand_evaluator poker_hand_evaluator

cargo test

docker stop poker_hand_evaluator

# where put this?
