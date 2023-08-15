init:
	maturin init

install:
	pip install -e requirements.txt

build:
	cargo build --release
	cp target/release/libpycalc_cli.so .

clean:
	# Remove the build .so file
	rm -f libpycalc_cli.so