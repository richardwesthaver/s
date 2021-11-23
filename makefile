# makefile --- shed makefile
RS:=build.rs Cargo.toml src rustfmt.toml
.PHONY: c

o:lisp;mkdir -p $@; \
cargo build --release;cp target/release/{shc,shs,she,shk,shd,shx} $@;cp -r $^ $@

i:o;install -pm 755 $</{shc,shs,she,shk,shd,shx} $(SHED)/bin

b:build.rs Cargo.toml src;cargo build

f:$(RS);cargo fmt

t:$(RS) tests;cargo test --all

c:;cargo clean;rm -rf o Cargo.lock _shc* shc.bash

#m:;shc meta -u 		# TODO 2021-10-26
