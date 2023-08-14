init:
	maturin init

build:
	cargo build --release
	cp target/release/libdata_conversion.so .

clean:
	# Remove the build .so file
	rm -f libdata_conversion.so