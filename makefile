# shed build scripts
o:build.rs;cargo install --path . --force --root $(SHED) --bin shed --target-dir $(STAMP)/shed -j8
c:;cargo clean;rm -rf $(shell which shed)
.PHONY: o c
