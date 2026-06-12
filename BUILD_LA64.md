# Building Python environment tools on LoongArch64 Platform

## Overview

This document describes how to build the Python environment tools on LoongArch64 architecture.

## Prerequisites

### System Requirements

- LoongArch64 architecture operating system
- **Rust** (LoongArch64 version)
- Git
- `musl-gcc` and `musl-dev` headers (optional, recommended — e.g. `apt install musl-tools` on Loongnix / UOS)

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

> **Note:** Building for musl requires `musl-gcc` and musl headers.
> On Loongnix / UOS: `apt install musl-tools`.

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
cargo test --release
```
