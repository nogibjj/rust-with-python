install:
	# Add polars
	cargo add polars -F lazy
	# Install python libraries
	pip install -r requirements.txt

build:
	cargo build --release

lint:
	cargo clippy

format:
	cargo fmt

test:
	cargo test

run:
	cargo run

rust-benchmark:
	@cargo build --release > /dev/null; \
	START=$$(date +%s.%N); \
	./target/release/polars-python-rust > /dev/null; \
	END=$$(date +%s.%N); \
	RUST_TIME=$$(echo "$$END - $$START" | bc -l); \
	printf "Rust elapsed time: %.5f seconds\n" $$RUST_TIME; \
	export RUST_TIME

python-pandas-benchmark:
	@START=$$(date +%s.%N); \
	python pandas-demo.py > /dev/null; \
	END=$$(date +%s.%N); \
	PANDAS_TIME=$$(echo "$$END - $$START" | bc -l); \
	printf "Python Pandas elapsed time: %.5f seconds\n" $$PANDAS_TIME; \
	export PANDAS_TIME

python-polars-benchmark:
	@START=$$(date +%s.%N); \
	python polars-demo.py > /dev/null; \
	END=$$(date +%s.%N); \
	POLARS_TIME=$$(echo "$$END - $$START" | bc -l); \
	printf "Python Polars elapsed time: %.5f seconds\n" $$POLARS_TIME; \
	export POLARS_TIME

benchmark: rust-benchmark python-pandas-benchmark python-polars-benchmark

.PHONY: benchmark rust-benchmark python-pandas-benchmark python-polars-benchmark