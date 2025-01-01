.PHONY: all
all:

.PHONY: seed
seed:
	bash ./scripts/seeding/seed.bash

lint:
	cargo clippy --all-targets --all-features -- -D warnings
