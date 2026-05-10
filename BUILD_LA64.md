# Building Python environment tools on LoongArch64 Platform

## Overview

This document describes how to build the Python environment tools on LoongArch64 architecture.

## Prerequisites

### System Requirements

- LoongArch64 architecture operating system
- **Rust** (LoongArch64 version)
- Git

## Build Steps

### 1. Get Source Code

```bash
git clone https://github.com/wubzbz/python-environment-tools-la64.git
cd python-environment-tools-la64
```

### 2. Build the Binary

```bash
cargo build --release --bin pet
```

The resulting binary will be at `target/release/pet`.

### 3. Testing and Verification

After building, you can run the test suite:

```bash
cargo test --release
```

