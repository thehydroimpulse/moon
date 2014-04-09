RUSTC ?= rustc
RUSTC_FLAGS ?=

BINS = moon

SRC = $(shell find src -name '*.rs')
BIN_TARGETS = $(patsubst %,target/%,$(BINS))
LIBCARGO = target/libmoon.timestamp

all: $(BIN_TARGETS)

bin: libmoon src/bin/bin.rs
	mkdir -p target && rustc -Ltarget --out-dir target src/bin/bin.rs

$(LIBCARGO): $(SRC)
	mkdir -p target
	$(RUSTC) $(RUSTC_FLAGS) --out-dir target src/moon/lib.rs
	touch $(LIBCARGO)

libmoon: $(LIBCARGO)

$(BIN_TARGETS): target/%: src/bin/%.rs $(LIBCARGO)
	$(RUSTC) $(RUSTC_FLAGS) -Ltarget --out-dir target $<

clean:
	rm -rf target

.PHONY: clean all libmoon