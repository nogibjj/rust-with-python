init:
	maturin init

build:
	cargo build --release
	cp target/release/libownership_pyrust.so .

clean:
	# Remove the build .so file
	rm -f libownership_pyrust.so