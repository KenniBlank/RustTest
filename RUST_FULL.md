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


        Integer Overflow:
        ```md
        Let’s say you have a variable of type u8 that can hold values between 0 and 255.
        If you try to change the variable to a value outside that range, such as 256, integer overflow will occur, which can result in one of two behaviors.
        When you’re compiling in debug mode, Rust includes checks for integer overflow that cause your program to **panic** at runtime if this behavior occurs.
        Rust uses the term panicking when a program exits with an error;

        When you’re compiling in release mode with the --release flag, Rust does not include checks for integer overflow that cause panics.
        Instead, if overflow occurs, Rust performs two’s complement wrapping.
        In short, values greater than the maximum value the type can hold “wrap around” to the minimum of the values the type can hold.

        To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:
        - Wrap in all modes with the wrapping_* methods, such as wrapping_add.
        - Return the None value if there is overflow with the checked_* methods.
        - Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
        - Saturate at the value’s minimum or maximum values with the saturating_* methods.
        ```

      - Floating-Point Types:

      Rust has two primary types for float numbers: **f32** and **f64**. All floating points are signed.

      **f64** has double precision. It is default on all modern cpu.
      ```rs
      fn main(){
          let x = 2.0;  // f64
          let y: f32 = 3.0; // f32
      }
      ```

      - Boolean Type
      ```rs
      fn main(){
          let t = true;
          let f: bool = false; // explicit type annotation
      }
      ```

      - Char Type

      In Rust, char uses single quote for definition
      ```rs
      fn main() {
          let c = 'z';
          let z: char = 'ℤ'; // with explicit type annotation
          let heart_eyed_cat = '😻';
      }
      ```

      - Basic Numeric Operators
      ```rs
      fn main(){
          let sum = 5 + 10;
          let difference = 95.5 - 4.3;
          let product = 4 * 30;
          let quotient = 56.7 / 32.2
          let truncated = -5 / 3; // Result is -1
          let remainder = 43 % 5;
      }
      ```

    - Compound Types:

      Compound types can group multiple values into one type. Rust has two primitive compound types:
      - Tuples

      Tuple is general way of grouping together a number of values with diffent types.
      > Tuple without value is called unit and is written as (). Represents empty value or empty return type.
      ```rs
        let tup: (i32, f64, char) = (500, 6.4, 'c');
        let tup = (500, 6.4, 'c');

        let (x, y, z) = tup;
        let x = tup.0, y = tup.1, z = tup.2;
      ```

      - Arrays

      This is another way to group data but unlike tuple, all values must be of same type.

      They are useful when you want your data allocated on the stack rather that the heap.
      ```rs
        let a = [1, 2, 3];
        let b = ["Jan", "Feb", "Mar"];
        let c: [i32: 5] = [1, 2, 3, 4, 5];
        let d = [3; 5]; // This means d = [3, 3, 3, 3, 3]
      ```
      > You are not allowed to access beyond score of array. Rust ensures memory safety.

  - Functions

    Rust doesn't care where function is defined.

    When defining function in rust, type must be specified for parameters.
    ```rs
    fn main() {
        print_labeled_measurement(5, 'h');
    }

    fn print_labeled_measurement(value: i32, unit_label: char) {
        println!("The measurement is: {value}{unit_label}");
    }
    ```

    - Expressions and Statements:

      Unlike other languages, rust has distinction between them.
      - Statements are instructions that perform some action and do not return a value.
      - Expressions evaluate to a resultant value. Let’s look at some examples.

      ```rs
      let x = (let y = 6); //results in error
      ```

      Expressions do not include ending semicolons.
      Expression is calling function, macro, etc.
      > Creating new scope block is expression too
      ```rs
      let x = {
          let y = 6;
          y + 1
      }
      // No Errors because y + 1 has result
      ```

    - Functions with return type:

    Return type must be specified too.
    If you write a semicolon, error will be raised because that's an statement
    ```rs
    fn five() -> i32{
        5
    }
    ```

  - Control Flow
    All conditions must be bool.

    - if expressions:
    ```rs
    if number < 0 {
        println!("Less than 0");
    } else if number == 5 {
        println!("Equal to 0");
    } else {
        println!("Greater than 0");
    }
    ```
      - Using if in a let statement:
      > Note that all values that have potential to be result must be of same type.
      ```rs
      let number = if true {5} else {6}; // Correct
      let number = if true {5} else {"false"}; // Raises error
      ```

    - Loops:
      - loop
      ```rs
      loop {
          println!("Hello");
      }
      ```

      - returning values from loops:

      Use break and value to return to
      ```rs
      let mut counter = 0;
      let result = loop {
          counter += 1;
          if counter == 10 {
              break counter * 2;
          }
      }
      ```

      - loop labels to disambiguate between multiple loops

      If you have loops within loops, break and continue apply to the innermost loop at that point.
      You can optionally specify a loop label  on a loop that you then can use with break or continue
      to specify that those keywords apply to the labeled loop instead of the innermost loop.

      ```rs
      let mut count = 0;
      'counting_up: loop {
          println!("count = {count}");
          let mut remaining = 10;

          loop {
              println!("remaining = {remaining}");
              if remaining == 9 {
                  break;
              }
              if count == 2 {
                  break 'counting_up;
              }
              remaining -= 1;
          }

          count += 1;
      }
      println!("End count = {count}");
      ```

      - while loop
      ```rs
      let mut number = 10;
      while number != 0{
          println!("{number}!");
          number -= 1;
      }
      ```

      - For Loop:

      In rust, for loop's safety and consiseness makes it the best loop (My opinion).
      ```rs
      let elements = [10, 100, 1000];

      for element in elements{
          println!("{element}");
      }
      ```

      In cases where you want to run certain times:
      ```rs
      for number in (1..4).rev(){
          println!("{number}");
      }
      ```
      > Note that the start of range in inclusive but not the end

  ## Ownership:

  It enables rust to make memory safety guarantees without needing a garbage collecctor.

  Some languages have garbage collection that regularly looks for no-longer-used memory as program runs.
  In other, programmer must explicitly allocate and free memory.

  Rust uses third approach: memory is managed through a system of ownership with a set of rules that compiler checks.

  ```md
  ### The Stack and the Heap

  Depending on whether value is on stack or heap affects how language behaves.

  Both stack and heap are parts of memory available to your code at runtime, but they are structured different.
  The stack stores values in order and removes the values in the opposite order. The heap is less organized: finds
  an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is address
  of that location.



  Stack: *Last In, First Out*
  - Adding data: pushing onto the stack,
  - Removing data: popping off the stack
  - All data stored on stack must have known, fixed size
  - Data with unknown size at compile time or size that might change must be store on the heap.

  Heap:
  - Allocation (Pointer concept)
  - Pointer to heap is fixed, you can store the pointer on the stack. When you want data, you follow the pointer.

  Pushing to stack is faster than allocating on heap because the allocator never has to serach for a place to store new data;
  location always on top of stack. Contemporary processors are faster if they jump around less in memory.

  When code calls function, values passed into function(including, potentially, pointers to data on heap) and function's local variables
  get pushed onto the stack. When function is over, those values get popped off the stack.

  Keeping track of what parts of code are using what data on the heap, minimizing the amount of duplicate data on the heap,
  and cleaning up unused data on the heap so you don’t run out of space are all problems that ownership addresses.

  Main purpose of ownership is to manage heap data.
  ```

  Ownership Rules:
  - Each value in Rust has an *owner*
  - Only one owner at a time
  - When owner goes out of scope, value will be dropped

  Remember to bring variable to scope only from point of use.

  - The String Type

  All previous data types have known fixed size which is stored on the stack.

  String is unique as the data is stored on the heap.
  ```rs
  let s = String::from("hello"); // :: operator
  ```

  Unlike String, string literals are hardcoded onto the program. Hence they are fast and efficient because they are immutable.

  For **String** type, in order to support mutable, growable piece of text, we need to allocate an amount of memory on the heap,
  which is unknown at compile time. This means:
  - The memory must be requested by allocator at runtime
  - We need a way of returning this memory to allocator when we're done with our **String**

  Rust take a different path for this: memory is automatically returned once variable that owns it goes out of scope.

  When vairable goes out of scope, rust calls a special function for us: **drop**


  Loop at the code below:
  ```rs
  let x = 5;
  let y = x;
  println!("{y}"); // No error

  let s1 = String::from("hello");
  let s2 = s1;
  println!("{s1}"); // Raises Error
  ```

  Firstly, x is set to 5. Then, the value of x is copied to y. Integers are simple and both the values are pushed onto the stack.

  In case of string however, the behaviour is totally different.

  String is made of three things:
  - pointer: points to the memory heap where value is stored
  - length
  - capacity

  When s1 is assigned to s2, the whole **string** data is copied i.e the pointer, length and the capacity. This mean that
  s2 and s1 points to same memory in the heap. Hence, when **drop** function is called, s1 is freed and compiler tries to free
  s2 too. This leads to *double free error*. Freeing memory twice may lead to memory corruption, which can lead to security
  vulneribilities.

  In rust, to ensure memory safety, after line **let s2 = s1**, rust considers s1 as no longer valid. So if you try to print
  s1, you will get an error at compile time.

  - Shallow copy: copying poiner, length, and capacity without copying data.
  - Deep Copy: move every things to new variable.

  Rust uses deep copy. By design, rust never auto creates "deep" copies of data so any automatic copying can be assumed to be
  inexpensive in terms of runtime performance.

  - VARIABLES AND DATA INTERACTING WITH CLONE
  > BUT if you did need to copy the heap data of a String, not just the stack data, we can use common method called *clone*.
  ```rs
  let s1 = String::from("hello");
  let s2 = s1.clone();

  println!("s1 = {s1}, s2 = {s2}");
  ```
  Just know that when you see call to **clone**, some arbitrary code is being executed and that code may be **expensive**.

  > Basic data types that are on stack need not be cloned, copies are made easily as this is cheap operation.

  Rust has a special annotation called **Copy** trait that we can place on types that are stored on the stack, as integers are.
  If type implements **Copy** trait, variables that use it do not move, but rather are trivially copied, making them valid after assignment
  to another variable. Rust doesn't allow us to annotate tpe with **Copy** trait if type, or any parts, has implemented the **Drop** trait.

  - OWNERSHIP AND FUNCTIONS:
  ```rs
  fn main() {
      let s = String::from("hello");  // s comes into scope

      takes_ownership(s);             // s's value moves into the function...
                                      // ... and so is no longer valid here

      let x = 5;                      // x comes into scope

      makes_copy(x);                  // x would move into the function,
                                      // but i32 is Copy, so it's okay to still
                                      // use x afterward

  } // Here, x goes out of scope, then s. But because s's value was moved, nothing
    // special happens.

  fn takes_ownership(some_string: String) { // some_string comes into scope
      println!("{some_string}");
  } // Here, some_string goes out of scope and `drop` is called. The backing
    // memory is freed.

  fn makes_copy(some_integer: i32) { // some_integer comes into scope
      println!("{some_integer}");
  } // Here, some_integer goes out of scope. Nothing special happens.
  ```

  - RETURN VALUES AND SCOPE:

  Returning values can be used to transfer ownership.
  ```rs
  fn main() {
      let s1 = gives_ownership();         // gives_ownership moves its return
                                          // value into s1

      let s2 = String::from("hello");     // s2 comes into scope

      let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                          // takes_and_gives_back, which also
                                          // moves its return value into s3
  } // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
    // happens. s1 goes out of scope and is dropped.

  fn gives_ownership() -> String {             // gives_ownership will move its
                                               // return value into the function
                                               // that calls it

      let some_string = String::from("yours"); // some_string comes into scope

      some_string                              // some_string is returned and
                                               // moves out to the calling
                                               // function
  }

  // This function takes a String and returns one
  fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                        // scope

      a_string  // a_string is returned and moves out to the calling function
  }
  ```

  > This is too tenuos so rust has "reference" same as C


  ## Regerences and Borrowing:
