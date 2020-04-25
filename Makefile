run:
	cargo run
test:
	cargo test -- --test-threads=1
clean:
	rm *.bc *.ll