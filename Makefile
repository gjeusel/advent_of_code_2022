.DEFAULT_GOAL := all

.PHONY: run
run:
	rustc main.rs
	./main

watch:
	cargo watch -x run
