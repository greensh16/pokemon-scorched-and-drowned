# climate.mk: builds the Rust climate library and exposes it to the main Makefile.
# Add `include climate.mk` near the top of the Makefile (after the toolchain variables).

CLIMATE_DIR    := climate-rs
CLIMATE_TARGET := thumbv4t-none-eabi
CLIMATE_LIB    := $(CLIMATE_DIR)/target/$(CLIMATE_TARGET)/release/libclimate.a
CLIMATE_SRCS   := $(shell find $(CLIMATE_DIR)/src -name '*.rs') \
                  $(CLIMATE_DIR)/Cargo.toml \
                  $(CLIMATE_DIR)/.cargo/config.toml \
                  $(CLIMATE_DIR)/rust-toolchain.toml

# Cargo does its own incremental tracking; Make just re-runs it when a source changes.
$(CLIMATE_LIB): $(CLIMATE_SRCS)
	cd $(CLIMATE_DIR) && cargo build --release \
		--target $(CLIMATE_TARGET) \
		-Zbuild-std=core

.PHONY: climate climate-test climate-header climate-check climate-clean

climate: $(CLIMATE_LIB)

# Host-side unit tests (pure logic + FFI against the mock C functions).
climate-test:
	cd $(CLIMATE_DIR) && cargo test

# Regenerate include/climate.h from src/ffi.rs (needs `cargo install cbindgen`).
climate-header:
	cd $(CLIMATE_DIR) && cbindgen --config cbindgen.toml --output ../include/climate.h

# Fails if the Rust library has any .data or .bss: C must own all state.
climate-check: $(CLIMATE_LIB)
	@if arm-none-eabi-objdump -h $(CLIMATE_LIB) \
		| awk '$$2 ~ /^\.(data|bss)/ && $$3 !~ /^0+$$/ { print; found = 1 } END { exit !found }' ; then \
		echo "climate-rs has non-empty .data/.bss: move that state into C vars"; exit 1; \
	else echo "climate-rs: no .data/.bss, OK"; fi

climate-clean:
	cd $(CLIMATE_DIR) && cargo clean
