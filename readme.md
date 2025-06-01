# RCAT 🦀📄

A minimal `cat` command-line clone written in Rust.

## 📦 What is it?

`rcat` is a basic implementation of the UNIX `cat` command, built using Rust.
It reads the contents of a file and prints them to standard output.

⚠️ **Note**: This project is still under development and may not handle all edge cases yet.

## 🛠️ Features

- ✅ Reads a file from a relative path
- ✅ Prints file contents to stdout
- ❌ Does NOT yet support multiple files or flags (like `-n`, `-E`, etc.)

## 📚 Usage

Run the program with a file path as an argument:
```
cargo run -- <file_path>
```

## 📁 How it works

1. Takes a single file path as an argument.
2. Resolves it to an absolute path based on the current working directory.
3. Reads the file contents using `std::fs::read()`.
4. Writes the contents to stdout.

## 🚧 Work in Progress

Planned improvements:

- Support multiple file inputs
- Better error messages
- Support reading from stdin (like the real `cat`)
- Add flag support (`-n`, `-b`, etc.)
