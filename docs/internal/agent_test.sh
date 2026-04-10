#!/usr/bin/env bash
./target/release/afoxd &
DAEMON_PID=$!
sleep 2

echo "--- 1. Navigating to Hacker News ---"
./target/release/afox open "https://news.ycombinator.com"

echo "--- 2. Snapping Initial Page ---"
./target/release/afox snap > /dev/null
echo "Initial Snap Complete."

echo "--- 3. Clicking Story e12 ---"
# e12 is the first story title link usually
./target/release/afox click e12

echo "--- 4. Snapping Post-Click Page ---"
# This will show if we successfully navigated
./target/release/afox snap | grep -E "url|title"

kill $DAEMON_PID
