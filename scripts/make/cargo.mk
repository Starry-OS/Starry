# Cargo features and build args

ifeq ($(V),1)
  verbose := -v
else ifeq ($(V),2)
  verbose := -vv
else
  verbose :=
endif

build_args-release := --release

build_args := \
  --target $(TARGET) \
  --target-dir $(CURDIR)/target \
  $(build_args-$(MODE)) \
  $(verbose)

RUSTFLAGS := -C link-arg=-T$(LD_SCRIPT) -C link-arg=-no-pie -C force-frame-pointers=yes
RUSTDOCFLAGS := --enable-index-page -Zunstable-options -D rustdoc::broken_intra_doc_links

ifeq ($(MAKECMDGOALS), doc_check_missing)
  RUSTDOCFLAGS += -D missing-docs
endif

define cargo_build
  $(call run_cmd,cargo build,$(build_args) $(1) --features "$(strip $(2))")
endef

clippy_args := -A clippy::new_without_default

define cargo_clippy
  $(call run_cmd,cargo clippy,--all-features --workspace $(1) $(verbose) -- $(clippy_args))
  $(call run_cmd,cargo clippy,-p axlog -p percpu -p percpu_macros $(1) $(verbose) -- $(clippy_args))
endef

all_packages := $(shell ls $(CURDIR)/crates)

define cargo_doc
  $(call run_cmd,cargo doc,--no-deps --all-features --workspace --exclude "arceos-*" $(verbose))
  @# run twice to fix broken hyperlinks
  @# for some crates, re-generate without `--all-features`
  $(call run_cmd,cargo doc,--no-deps -p percpu $(verbose))
endef
