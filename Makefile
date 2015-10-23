
all:
	cargo build --release
	cp ./target/release/httpd ./httpd
