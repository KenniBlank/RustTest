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

## Cargo
Cargo is Rust's build system and package manager.

Cargo handles a lot of tasks for you: building code, downloading libraries, and building the libraries.<br>
(Libraries are called dependencies)

- Creating a project with Cargo
    ```bash
    cargo new cargo_name
    ```
    This creates its files in a directory of the same name.

    The directory will have two files:
    - cargo.toml file
    - src directory with main.rs file inside
    It has also generated new git repo along with .gitignore file. This won't be generated if cargo new is runned within existing Git Repository; This behaviour can be overridden by:
    ```bash
    cargo new --vcs=git cargo_name
    ```
    
    Also, you can run (to get all options):
    ```bash
    cargo help
    ```



