# The hash-gen utility
![](.github/assets/header.png)

This utility is a command-line tool for generating cryptographic hashes of files using algorithms like MD5, SHA-1, SHA-256, and SHA-512.

## Features

- Supports multiple hashing algorithms: MD5, SHA-1, SHA-256, and SHA-512.
- Extensible to support more hashing algorithms in the future.
- Simple and intuitive command-line interface.

## Installation

### Via Cargo

You can easily install the utility directly from [crates.io](https://crates.io) using Cargo, Rust’s package manager. To install the latest version, run:
```bash
cargo install hash-gen
```
This command will download, compile, and install the utility to your Cargo bin directory (~/.cargo/bin by default). Ensure that this directory is in your system’s PATH environment variable to run the command from anywhere.

### From source

Alternatively, if you prefer to build the utility from source, you can clone the repository and build it manually:
```bash
git clone https://github.com/0xlay/hash-gen.git
cd hash-gen
cargo build --release
```
The binary will be available in the target/release directory.

## Usage
Once installed, you can use the utility from the command line to generate cryptographic hashes for files using different algorithms.

### Basic Commands

MD5 hash
```bash
hash-gen md5 --path /path/to/file.txt
```
SHA1 hash
```bash
hash-gen sha1 --path /path/to/file.txt
```
SHA256 hash
```bash
hash-gen sha256 --path /path/to/file.txt
```
SHA512 hash
```bash
hash-gen sha512 --path /path/to/file.txt
```

## Generate documentation
You can generate the project’s documentation using Rust’s built-in documentation tool, cargo doc. Run the following command:
```bash
cargo doc --open
```
This will generate the documentation and open it in your web browser. You’ll find detailed descriptions of all structs, traits, and methods available in the project.

## Contributing

We welcome contributions to this project. If you’d like to contribute:
1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request.

We are happy to review any proposed changes.

## License

This project is licensed under the MIT license - see the LICENSE file for details.
