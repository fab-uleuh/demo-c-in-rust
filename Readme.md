# Rust-C Interoperability Demo

This project demonstrates how to call C code from Rust, showcasing the interoperability between these two languages.

## Overview

This demo project illustrates:
- How to create and use C functions in Rust
- Setting up the necessary build configuration
- Handling data types between C and Rust
- Managing memory safety across the language boundary

## Project Structure

The project contains:
- `src/main.rs` - Main Rust entry point
- `src/c_bindings.rs` - Rust bindings for C functions
- `src/c/` - C source files
- `build.rs` - Build script for compiling C code
- `Cargo.toml` - Project configuration and dependencies

## Building and Running

To build and run the project:

1. Make sure you have Rust and Cargo installed:
   ```bash
   rustc --version
   cargo --version
   ```

2. Build the project:
   ```bash
   cargo build
   ```

3. Run the project:
   ```bash
   cargo run
   ```

## Requirements

- Rust toolchain (latest stable version)
- C compiler (gcc/clang)
- cc crate (automatically managed by Cargo)

## Safety Considerations

When calling C code from Rust, it's important to:
- Properly handle unsafe code blocks
- Manage memory correctly across the language boundary
- Validate data types and conversions
- Handle potential null pointers and undefined behavior

## License

MIT License
