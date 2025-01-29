#!/bin/bash

echo "Running evaluation library tests"
cargo nextest run --lib --no-fail-fast
