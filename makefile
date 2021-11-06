# makefile --- shed makefile
RS:=build.rs Cargo.toml src rustfmt.toml

.PHONY: c

o:etc lisp;mkdir -p $@; \
cargo build --release;cp target/release/shc $@;cp -r $^ $@

i:o;install -pm 755 $</shc $(SHED)/bin

b:build.rs Cargo.toml src;cargo build

f:$(RS);cargo fmt

t:$(RS) tests;cargo test --all

c:;cargo clean;rm -rf o Cargo.lock

#m:;shc meta -u 		# TODO 2021-10-26
