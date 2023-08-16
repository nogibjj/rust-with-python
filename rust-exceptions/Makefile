init:
	maturin init

install:
	pip install -e requirements.txt

build:
	cargo build --release
	cp target/release/librust_exceptions.so .

clean:
	# Remove the build .so file
	rm -f librust_exceptions.so