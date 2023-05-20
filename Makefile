clean:
	cargo fix --allow-dirty

fmt:
	rustfmt src/*

run:
	cargo run

cfr: clean fmt run