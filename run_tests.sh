#!/bin/sh

cargo test | awk '!/running 0/ && !/0 passed; 0 failed; 0 ignored; 0/ {print}'
