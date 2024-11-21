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

## Libraries

To obtain user input and then print result as output, we need to bring the **io** input/output library into scope.

The *io* library comes from the standard library, knowd as *std*:
```bash
use std::io;
```
The set of items defined in the standard library that it brings into the scope of every program is ***prelude***

### Variables and Mutability:
- By Default, all variables are immutable (Once given value, value can't be changed)
- To make variables mutable use **mut**.

```rs
let mut guess = String::new();
```
Here, guess is bound to the result of calling *String::new*, a function that returns a new instance of a *String*.
The **::** syntax in the **::new** line indicated that **new** is an associated function of the *String* type.

In full, the line has created a mutable variable that is currently bound to a new, empty instance of a *String*.

- Receiving user Input:
```rs
std::io::stdin()
    .read_line(&mut guess)
    .expect("Failed to read line");
```
In the first line, stdin function returns an instance of *std::io::Stdin*.

The second line calls the *read_line* method on the standard input handle to get input from the user.
We're also passing *&mut guess* as an argument to *read_line*. The **&** indicates that this argument is a reference
> *&mut guess* should be passed instead of *&guess* to make it mutable

The third line is handling potential Failure

The *read_line* puts whatever user enters into the string we pass to it, but it also returns a **Result** value.
**Result** is a enumeration, which is a type that can be in one of multiple possible states. Each state is a variant.

*Result*'s varians are *Ok* and *Err*. An instance of *Result* has and **expect** method that your call. If this instance of *Result* is *err* value,
*expect* will cause the program to crash and display the message that you passed as an argument to *expect*


## Using Crates:
Simply put the library and its version in the .toml file:
```toml
[dependencies]
rand = "0.8.5"
```
If you need to update the libraries:
```bash
cargo update
```
this will update the dependencies to latest but less that version change. In this case: >0.8.5 but <0.9.0


You can't remember what crates have so:
```bash
crate doc --open
# List all the dependencies that the current program runs on with its documentation.
```


## Common Programming Concepts in Rust:
  - Variables and Mutability

    All variables are immutable by default which need to be set to be mutable with *mut* keyword.
    This is for making code more convenient to write.

    ```rs
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    let x = 4;
    let mut y = 5;
    ```

    Shadowing Variables:

      This is basically redefining same variables with maybe different variable type

  - Data Types

    > In Rust, all variables must have a type annotation.

    There are two data types: scalar and compound.

    - Scalar Types:

      A scalar type represents a single value. Rut has four primary scalar types: integers, floating points, booleans, and characters.

      - Integer Types: Number without fractional compound.

        ![Integers in Rust](/imgNotes/integers.png)
        Each variant can be either signed or unsigned and has explicit size.

        Signed variant: -(2^(n-1)) to 2^(n-1) - 1 inclusive.

        Unsigned variants: 0 to 2^(n) -1

        Note: isize and usize depend on the architecture of the computer, the program runs in.

        You use isize and usize when indexing some sort of collection.

        In Rust, integer literals are used to represent integer values directly in the code.
        They can be specifie in several different number system.
        > You can include *underscore* for better readability.
        ```rs
        let decimal: i32 = 42; //Decimal Integer
        let octal: i32 = 0o_52; // Octal for 42
        let hex: i32 = 0x_2A; // Hexadecimal for 42
        let binary: i32 = 0b_101010; // Binary for 42

        let addition = decimal + 10i32; // Adds 10 to decimal variable and store it
        ```

        ```md
        # Integer overflow

        ```




  - Functions
  - Comments
  - Control Flow
