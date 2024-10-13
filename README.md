# PixieOS

PixieOS is a minimal operating system kernel written in Rust. This project aims to create a basic OS from scratch, demonstrating low-level programming concepts and Rust's capabilities in systems programming.

## Features

- Bare-metal Rust implementation
- Custom target specification for x86_64 architecture
- VGA text mode output
- Panic handler implementation

## Prerequisites

To build and run PixieOS, you need:

- Rust nightly toolchain
- `cargo-bootimage` for creating bootable disk images
- QEMU for running the OS (optional, but recommended for testing)

## Setup

1. Install the Rust nightly toolchain:
   ```
   rustup override set nightly
   ```

2. Install `cargo-bootimage`:
   ```
   cargo install bootimage
   ```

3. Install QEMU (if not already installed)

## Building

To build the project, run:

```rs
cargo build
```

## Running

To create a bootable image and run it in QEMU, use:

```rs
cargo run
```


## Project Structure

- `src/main.rs`: Contains the kernel entry point and basic VGA buffer output
- `Cargo.toml`: Rust package manifest
- `rust-toolchain`: Specifies the Rust toolchain version
- `.cargo/config.toml`: Cargo configuration for custom target and runner
- `x86_64-pixie_os.json`: Custom target specification for x86_64

## Contributing

Contributions to PixieOS are welcome! Please feel free to submit pull requests, create issues or suggest improvements.

## License

MIT

## Author

Wbert Adrian Castro Vera <dobleuber@gmail.com>
