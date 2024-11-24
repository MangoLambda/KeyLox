## Disclaimer

This project is currently in the early stages of development and is intended for experimental and educational purposes only. Use it at your own risk.

## Overview

KeyLox is a Terminal User Interface (TUI) based password manager designed to securely store and manage your credentials. It leverages the Argon2 algorithm for password derivation and Aegis256 for encrypting your credentials.

## Features

- **Secure Password Storage**: Uses Argon2 for password password based key derivation and Aegis256 for encryption.
- **TUI Interface**: User-friendly terminal interface for managing passwords.
- **Cross-Platform**: Works on various operating systems including Windows, macOS, and Linux.

## Installation

To install KeyLox, you need to have Rust installed. You can install Rust from [here](https://www.rust-lang.org/tools/install).

Clone the repository and build the project:

```sh
git clone https://github.com/MangoLambda/KeyLox.git
cd KeyLox
cargo build --release
```

## Usage

Run the application using:

```sh
cargo run --release
```

Follow the on-screen instructions to add, view, and manage your passwords.

## Contributing

Contributions are welcome! Please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgements

- [Argon2](https://github.com/P-H-C/phc-winner-argon2)
- [Aegis256](https://github.com/jedisct1/aegis)
