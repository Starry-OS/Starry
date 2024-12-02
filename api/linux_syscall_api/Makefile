ARCH ?= x86_64

# Target
ifeq ($(ARCH), x86_64)
  TARGET := x86_64-unknown-none
else ifeq ($(ARCH), riscv64)
  TARGET := riscv64gc-unknown-none-elf
else ifeq ($(ARCH), aarch64)
  TARGET := aarch64-unknown-none
endif


define run_cmd
  @printf '$(WHITE_C)$(1)$(END_C) $(GRAY_C)$(2)$(END_C)\n'
  @$(1) $(2)
endef


doc_check_missing:
	cargo update
	cargo update --precise 0.4.19 log
	$(call run_cmd,cargo doc,--no-deps --all-features --workspace)
