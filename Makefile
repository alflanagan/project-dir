.PHONY: clippy

clippy:
	cargo clippy -- -W clippy::pedantic

tasks:
	@grep -o '^[a-zA-Z]\+' Makefile | grep -v tasks
