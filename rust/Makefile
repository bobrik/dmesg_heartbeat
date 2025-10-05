KDIR = /lib/modules/$(shell uname -r)/build
RUSTC_VERSION = $(shell grep CONFIG_RUSTC_VERSION_TEXT: $(KDIR)/rust/uapi/uapi_generated.rs | sed 's/.*rustc \([^ ]\+\) .*/\1/')

.DEFAULT_GOAL := build

.PHONY: rust-toolchain.toml
rust-toolchain.toml:
	@echo '[toolchain]'                   > rust-toolchain.toml
	@echo 'channel = "$(RUSTC_VERSION)"' >> rust-toolchain.toml

build: rust-toolchain.toml
	make -C $(KDIR) M=$$PWD

clean:
	make -C $(KDIR) M=$$PWD clean
