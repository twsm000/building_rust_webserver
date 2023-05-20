clean:
	cargo fix --allow-dirty

fmt:
	rustfmt src/**/*.rs

run:
	cargo run

cfr: clean fmt run