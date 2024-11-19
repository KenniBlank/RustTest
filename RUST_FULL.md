# Rust

Rust is a statically typed language.

I am learning the language because I know Python as a dynamically typed language but I don't know a future proof statically typed language.

## Anatomy of a Rust Program:
```rs
fn main(parameters...){
    println!("Hello World!");
    // Code...
}
```
- Rust has special function **main** which is always the first code that runs in every executable Rust program.

- Rust Style for indentation is 4 spaces.

- macro don't always follow the same rules as functions: macro is called with !, function is not.

- end statement with semicolon(;)

> If you want to auto format the program, use **rustfmt**.

## Cargo and it's Anatomy
Cargo is Rust's build system and package manager.

Cargo handles a lot of tasks for you: building code, downloading libraries, and building the libraries.<br>
(Libraries are called dependencies)

- Creating a project with Cargo
    ```bash
    cargo new cargo_name
    ```
The directory "cargo_name" will have two files:
- cargo.toml file
- src directory with main.rs file inside

It has also generated new git repo along with .gitignore file.
This won't be generated if cargo new is runned within existing Git Repository.
This behaviour can be overridden.

### Anatomy of .toml format for Rust
```toml
[package]
name = "cargo_name"
version = "0.1.0"
edition = "2021"

[dependencies]
```
The first line, *[package]*, is a section heading that indicates that the following statements are configuring a package.
The Last line, *[dependencies]* is start of section for you to list any of your project's dependencies.
The packages of code are called **crates** in rust.

Cargo expects all code to be inside src/ directory. It has place designated for everything.

- We can create a project using ***cargo new***
- We can build a project using ***cargo build***
- We can build and run a project in one step using ***cargo run***
- We can build a project without producing a binary to check for errors using ***cargo check***
- We can build project for release using ***cargo build --release*** which has optimizations: stored at *target/release*
