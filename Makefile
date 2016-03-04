test:
	cargo test

test-out:
	cargo test -- --nocapture

generate:
	python bindings/generate_bindings.py nvim src
