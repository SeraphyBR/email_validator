#!/bin/sh
sqlx database create && sqlx migrate run
cargo run --release
