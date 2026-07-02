default:
    @just --list

build:
    cargo build

release:
    cargo build --release

run:
    cargo run

fmt:
    cargo fmt

lint:
    cargo clippy

check:
    cargo fmt --check && cargo clippy

deploy:
    git pull && cargo build --release && sudo ./target/release/eipi
