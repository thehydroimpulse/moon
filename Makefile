RUSTC ?= rustc
RUSTC_FLAGS ?=

BINS = moon

SRC = $(shell find src -name '*.rs')

all: libmoon moon test

moon: libmoon
	mkdir -p target
	$(RUSTC) $(RUST_FLAGS) -Ltarget --out-dir target src/bin/moon.rs

libmoon: $(SRC)
	mkdir -p target
	$(RUSTC) $(RUSTC_FLAGS) --out-dir target src/moon/lib.rs

test: $(SRC)
	mkdir -p target
	$(RUSTC) $(RUSTC_FLAGS) --test --out-dir target src/moon/lib.rs
	./target/moon

clean:
	rm -rf target

.PHONY: clean all libmoon
