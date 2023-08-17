install:
	#add derive feature to clap (cli tool)
	cargo add clap --features derive
	#add polars
	cargo add polars --features lazy

build:
	cargo build --release

lint:
	cargo clippy

format:
	cargo fmt

test:
	cargo test

run:
	#runs the cli
	cargo run --

bench:
	#view the result:  target/criterion/report/index.html
	cargo bench