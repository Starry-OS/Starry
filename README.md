# Starry

## Introduction

The main repository of Starry-OS, which will assemble all kernel components into a kernel according to a certain configuration.

This repository is based on [Starry](https://github.com/Arceos-monolithic/Starry), and remove submodules from the kernel, placing them [here](https://github.com/orgs/Starry-OS/repositories).To use these crates in the local workspace, please see the [Pull crates to local workspace](#pull-crates-to-local-workspace) section.

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

$ ./build_img.sh -a x86_64

$ make A=apps/monolithic_userboot FEATURES=img ARCH=x86_64 run

# Run the testcases in the monolithic architecture

$ make A=apps/monolithic_userboot APP_FEATURES=batch FEATURES=img,sched_rr ARCH=x86_64 NET=y BLK=y run
```

## Build and run testcases with ext4fs
The document of the ext4fs is [here](./doc/ext4fs.md).
```sh
# Build the image in ext4fs
$ ./build_img.sh -a x86_64 -fs ext4

# Run in the lwext4fs with Rust interface, whose url is https://github.com/elliott10/lwext4_rust.
$ make A=apps/monolithic_userboot FEATURES=lwext4_rust LOG=off NET=y BLK=y ACCEL=n run

# Run in a new ext4fs written in Rust, whose url is https://github.com/yuoo655/ext4_rs.
$ make A=apps/monolithic_userboot FEATURES=ext4_rs LOG=off NET=y BLK=y ACCEL=n run

# Replace virt-io with ram-disk
$ make A=apps/monolithic_userboot FEATURES=ext4_rs,img LOG=error NET=y BLK=y ARCH=x86_64 ACCEL=n run

# Run testcases for OSCOMP
$ make A=apps/monolithic_userboot FEATURES=ext4_rs,img LOG=off NET=y BLK=y ACCEL=n run APP_FEATURES=batch
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

## Run with e1000 driver
```sh
$ make A=apps/monolithic_userboot FEATURES=img,sched_rr,e1000_net LOG=off ACCEL=n APP_FEATURES=batch NET=y BLK=y run 
```

## Run ZLMediaKit
```sh
$ ./build_img.sh -a x86_64 -fs ext4  -s 80 -file x86_64_ZLM
$ make A=apps/monolithic_userboot FEATURES=another_ext4,img,sched_rr LOG=error NET=y BLK=y ARCH=x86_64 ACCEL=n run
```

## Notes

- Please remove unnecessary dependencies in `Cargo.toml` before your commit.
- After pulling a new crate to the local workspace, maybe you need to execute `make clean` to update the cache.

