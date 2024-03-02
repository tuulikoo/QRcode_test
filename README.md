# QRcode_test
[Github](https://github.com/tuulikoo/QRcode_test)
CLI-app for testing QRcode generation.

Uses crates [qrcode v0.13.0](https://crates.io/crates/qrcode) for QR-code generation and [sha256](https://crates.io/crates/sha256) for hash-function.

## Usage
Generates a given string by default to the command line.
- String is set by running with 
```bash
cargo run `<input string>`
```

or

```bash
cargo run -- --png`<input string>`
```

for example "cargo run -- --png testing qr-code generation"
- If no input string is given, a default string is used
- The appearance of the generated QR code in the command line depends on whether a dark or light theme is used in the editor.

Make sure to replace `<input string>` with the actual input string you want to use.

![Alt text](<Screenshot 2024-01-30 at 11.10.39.png>)

An image is also generated to folder img. Image name is your given input string hashed with sha256. Image can be opened by clicking and choosing "open with live server".

![Alt text](<Screenshot 2024-01-30 at 11.10.50.png>)

## Tests
Run tests with the following commands: 

```bash
cargo test
```

or
```bash
cargo tarpaulin
```

No input string is required for running tests.