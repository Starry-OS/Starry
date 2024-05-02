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

$ ./build_img.sh x86_64

$ make A=apps/monolithic_userboot ARCH=x86_64 run
```

## Build C applications

```sh
# Load axlibc
$ make pre_libc

# Build the application
# $ make A=apps/<app_name> ARCH=<arch> build
$ make A=apps/c/helloworld ARCH=x86_64 build
```



* 项目最终呈现形态是什么形式。给三种输入：Word 带截图、Excel 进去 Excel 出来，Excel 包括 去除无效链接 和 标记有效和无效的链接
* 日常查删的大平台是没问题  但是快手在网页端打开是说 网页不支持图片的显示  但是从手机端打开  可能是无法显示  可能是已经被删掉了
* 嫁到学校的服务器上  学校的服务器有什么具体的要求   给个指标性的要求
* 有时候找到的链接可能是懂车帝。是今日头条的衍生平台，在今日头条发布的一瞬间会同步到懂车帝。但是懂车帝的链接无法被打开。 看看懂车帝是否属于僵尸链接，还是确实可以通过某种方式来打开。如果可以的话，可以给出打开的方式。
