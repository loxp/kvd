.PHONY: all build test bench clean clean_tmp

all: build test bench clean_tmp

build:
	cargo build --all

test:
	cargo test --all

bench:
	cargo bench --all

clean:
	cargo clean

clean_tmp:
	rm -rf /tmp/kvd_store

