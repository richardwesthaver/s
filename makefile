# makefile --- shed makefile
RS:=build.rs Cargo.toml src rustfmt.toml

.PHONY: b t f c m

o:lisp/shed.el $(RS);mkdir -p $@; \
cargo build --release;cp target/release/shc $@;cp $< $@

i:o;install -pm 755 $</shc $(SHED)/bin

b:build.rs Cargo.toml src;cargo build

f:$(RS);cargo fmt

t:$(RS) tests;cargo test --all

c:;cargo clean;rm -rf o Cargo.lock

m:;shc meta -u 		# TODO 2021-10-26
