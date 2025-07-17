# unAEF

A simple, fast, and cross-platform command-line tool for decrypting EvotorOS Over-The-Air `.aef` files.

### 📋 Prerequisites

You need to have the Rust toolchain (including `rustc` and `cargo`) installed on your system. You can easily install it via [rustup.rs](https://rustup.rs/).

### 🛠️ Building

Clone the repository and build the project with Cargo using the `--release` flag for an optimized binary.


## 1. Clone the repository
```
git clone https://github.com/gentoocat/unAEF.git
```
## 2. Navigate into the project directory
```
cd unAEF
```
## 3. Build the project
```
cargo build --release
```
The compiled executable will be located at `target/release/unaef` 

## 🚀 Usage

Run the program from your terminal, providing the input and output file paths.

```bash
unaef -i encrypted.aef -o decrypted.bin
```

After decrypting use [7-Zip archivator](https://www.7-zip.org) to unpack decrypted archive.

## 📜 License

This project is licensed under the Apache 2.0 License

---

## Note to AOSP devs

Зачем было воровать ее у Атола? Во время реверс-инженеринга я много раз встречался с ru.atol. Вы просто взяли их механизм обновления и к себе забрали.

