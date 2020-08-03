#!/bin/bash

cargo build --release --bin game
./target/release/game > /dev/shm/game.log
tail -n 1000 /dev/shm/game.log
echo
grep "Expected" /dev/shm/game.log
echo
echo "/dev/shm/game.log"
echo
