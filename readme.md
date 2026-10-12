# SWOS Kernel

```
 /\_/\
( o.o )  SWOS Kernel - High Performance & Safe
 > ^ <   Commercial Kernel Written in Rust
```

SWOS Kernel is a commercial-grade system kernel designed with performance, safety, and modern hardware architectures in mind. Built entirely from the ground up using **Rust**, it leverages zero-cost abstractions and strict memory safety guarantees to provide a resilient core foundation.

## Technical Overview

| Feature / Item   | Details                        |
| :--------------- | :----------------------------- |
| **Project Name** | SWOS Kernel                    |
| **Language**     | Rust                           |
| **Type**         | Commercial Kernel              |
| **Maintainers**  | SnakeWolf, SnakeWolf Developer |
| **Status**       | Beta (Active Development)      |

## Key Features

- **Memory Safety**: Pure Rust implementation preventing common buffer overflows and data races.
- **Modular Architecture**: Clean separation between core scheduling, memory management, and device drivers.
- **Commercial Support**: Built for high-reliability target environments and scalable hardware configurations.

## Prerequisites

Ensure you have the following dependencies installed on your build system:

- **Rust Toolchain**: `rustup` with the latest nightly channel installed.
- **Build System**: `make` and GCC/Clang cross-compilation toolchains.
- **Emulator**: `QEMU` (required for local quick testing and emulation).

## Quick Start Guide

Open your terminal in the project root directory and run the relevant build commands below.

### 1. Build Entire Project

To compile all kernel components and produce output binaries:

```sh
make all

```

### 2. Run in Emulator

To launch the built kernel directly inside a local QEMU instance:

```sh
make run

```

## Disclaimer & Project Status

> [!note]
> Pre-release Version

> [!Warning]
> This project is currently in **Beta**. APIs and internal architectures may change without prior notice. Use caution when testing in target environments.
