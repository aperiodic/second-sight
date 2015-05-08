#!/bin/sh

set -e

cargo build
Xephyr :1 &
DISPLAY=:1 cargo run &
