test:
	mkdir -p bin && rustc --test -g -o bin/test src/lib.rs
	./bin/test

build:
	mkdir -p bin && rustc -g -o bin/gc src/bin.rs
	./bin/gc

.PHONY: test build