#!/bin/sh

cargo test 2>/dev/null | awk 'NF && !/running 0/ && !/0 passed; 0 failed; 0 ignored; 0/'
