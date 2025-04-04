.PHONY: all check format lint test test-coverage build test-coverage

all: check format lint test-coverage build 

check:
	cargo check

format: 
	cargo fmt

lint: 
	cargo clippy

test: 
	cargo test

test-coverage:
	cargo tarpaulin --out Html		

build:
	cargo build --release
