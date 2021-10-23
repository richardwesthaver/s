# makefile --- shed makefile
o:lisp/shed.el build.rs Cargo.toml src;mkdir -p $@;cargo build --release;cp target/release/shed $@;cp $< $@
b:build.rs Cargo.toml src;cargo build
f:rustfmt.toml;cargo fmt --all
t:;cargo test --all
c:;cargo clean;rm -rf o Cargo.lock
i:o;install -pm 755 $</shed $(SHED)/bin
w:
.PHONY: f t c i
