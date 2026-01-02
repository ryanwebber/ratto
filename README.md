# Ratto

The start of a tiny operating system written in Rust, built as a learning exercise.

## Workspace

 * `ratto-core` - Core library with architecture-independent code, traits, and abstractions, in a testable library.
 * `ratto-kernel` - The kernel implementation, with architecture-specific modules.
 * `ratto-entry` - The boot and entry code, with architecture-specific startup code.
 * `ratto-qemu` - QEMU-specific code for running Ratto in QEMU.
 * `ratto-xtask` - A tool for building and running Ratto which deals with cross-compilation, rust flags, linking,
   assembling, image creation, etc.

# Getting Started

Build and run Ratto in QEMU:

```sh
cargo xtask run --config-path config/raspi3.config
```
