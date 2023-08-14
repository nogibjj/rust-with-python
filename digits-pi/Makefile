init:
	maturin init

build:
	cargo build --release
	cp target/release/libdigits_pi.so .

clean:
	# Remove the build .so file
	rm -f libdigits_pi.so