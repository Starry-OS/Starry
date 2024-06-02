# Starry

## Introduction

The main repository of Starry-OS, which will assemble all kernel components into a kernel according to a certain configuration.

## Structure

![avatar](./doc/figures/Starry.svg)

## Build and run

```sh
# Run in unikernel architecture

# $ make A=apps/<app_name> ARCH=<arch> run

# The <app_name> is the application stored in the ./apps folder.

# The <arch> can be x86_64, risc64 and aarch64.

$ make A=apps/helloworld ARCH=x86_64 run

# Run in monolithic architecture

# Make the testcases image first

# $ ./build_img.sh <arch>

$ ./build_img.sh -m x86_64

$ make A=apps/monolithic_userboot ARCH=x86_64 run
```

## Pull crates to local workspace

```sh
# To download the tool
$ cargo install kbuild

$ mkdir crates

# Load crates
$ kbuild patch add linux_syscall_api

$ kbuild patch add axstarry

# Then crates will be downloaded to the crates/ folder

# To remove the crates
$ kbuild patch remove linux_syscall_api

$ kbuild patch remove axstarry

# Please verify that crates don't have any uncommitted changes before removing them.

```

## Notes

- Please remove unnecessary dependencies in `Cargo.toml` before your commit.
- After pulling a new crate to the local workspace, maybe you need to execute `make clean` to update the cache.

