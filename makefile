# shed build scripts
o:build.rs src;cargo install --path . --force --root $(SHED) --bin shed
c:;cargo clean;rm -rf $(shell which shed)
f:rustfmt.toml;cargo fmt --all
.PHONY: c f
