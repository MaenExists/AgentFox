#!/usr/bin/env bash
./target/release/afoxd &
DAEMON_PID=$!
sleep 2

echo "--- Benchmarking Open ---"
time ./target/release/afox open "https://example.com"

echo "--- Benchmarking Snap (Turbo) ---"
time ./target/release/afox snap > /dev/null

echo "--- Benchmarking Click (Direct ID) ---"
# We know e1 is the first link on example.com
time ./target/release/afox click e1

kill $DAEMON_PID
