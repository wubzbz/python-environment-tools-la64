# Building Python environment tools on LoongArch64 Platform

## Overview

This document describes how to build the Python environment tools on LoongArch64 architecture.

## Prerequisites

### System Requirements

- LoongArch64 architecture operating system
- **Rust** (LoongArch64 version)
- Git

The musl target uses Rust's self-contained CRT and libc, so a separate
`musl-gcc` toolchain is not required.

## Build Steps

### 1. Get Source Code

```bash
git clone https://github.com/wubzbz/python-environment-tools-la64.git
cd python-environment-tools-la64
```

### 2. Build the Binary (Recommended: musl)

To ensure the binary runs on any LoongArch64 system regardless of the system glibc version,
we recommend building against musl for static linking:

```bash
# Install the musl target
rustup target add loongarch64-unknown-linux-musl

# Build
cargo build --release --target loongarch64-unknown-linux-musl --bin pet
```

The resulting binary will be at `target/loongarch64-unknown-linux-musl/release/pet`.
It is statically linked against musl — no external libc dependency.

The repository's `.cargo/config.toml` enables `crt-static` for this target.
Keep that setting: without it, Cargo may produce an ELF that requests the GNU
dynamic loader despite using the musl target.

Verify the artifact before publishing:

```bash
file target/loongarch64-unknown-linux-musl/release/pet
readelf -d target/loongarch64-unknown-linux-musl/release/pet
```

`file` must report `statically linked`, and `readelf -d` must not list a
`NEEDED` shared library or a program interpreter.

### Alternative: gnu target

If musl is not available, you can still build against the system glibc:

```bash
cargo build --release --bin pet
```

The resulting binary will be at `target/release/pet`.

> **⚠️ Compatibility warning:** This binary dynamically links against your system glibc.
> It will only run on systems with a glibc version equal to or newer than the build host.
> Run `ldd --version` at build time to check your glibc version and ensure the target system meets it.

### 3. Testing and Verification

After building, you can run the test suite:

```bash
cargo test --release --target loongarch64-unknown-linux-musl
```
