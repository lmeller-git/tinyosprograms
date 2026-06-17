# tinyosprograms

Userspace programs, utilities, and examples included in the tinyOS image.

## Overview

This repository acts as the userspace payload for **[tinyOS](https://github.com/lmeller-git/tinyos)**. It contains various user-facing applications, system utilities, and example templates that are compiled and loaded directly into the tinyOS RamFS during the kernel's image generation process.

Whether you are looking to explore how applications run on tinyOS or want to build your own custom userspace tools, the examples here provide the standard reference.

---

## Anatomy of a tinyOS Program

The tinyOS build orchestrator expects a specific directory structure to successfully locate, compile, and package userspace binaries.

To be recognized as a valid program, your application must meet the following criteria:

1. **Location:** It must reside in its own dedicated subdirectory within the `programs/` folder (e.g., `programs/my_app/`).
2. **Build Script:** It must contain an executable bash script named `build.sh`.
3. **Compilation Output:** When executed, `build.sh` must compile the source code and produce a final, linked executable strictly named `a.out` that is compatible with the `tinyos_abi`.

### Examples

You can explore the `programs/example-*` directories to see exactly how Rust, C or ASM applications may be structured, linked against `libtinyos` and `libtinygraphics`, and compiled into valid binaries.

---

## Usage & Integration

Programs contained in this repository are designed to be natively integrated with the tinyOS ecosystem.

You typically do not need to build these programs manually. When you trigger a build (`make all` or `make run`) from the main **tinyOS** repository, the build system automatically:
1. Iterates through the `programs/` directory in this repository.
2. Executes each target's `build.sh` script.
3. Bundles the resulting `a.out` executables into the kernel's initial RamFS.

---

## Quick Links

* **[tinyOS Core](https://github.com/lmeller-git/tinyos)** — The main operating system kernel and build orchestrator.
* **[libtinyos](https://github.com/lmeller-git/libtinyos)** — The core userspace libraries, ABI wrappers, and C bindings used to write these programs.
* **[tinyTerm](https://github.com/lmeller-git/tinyterm)** — The default graphical terminal emulator and interactive shell.

