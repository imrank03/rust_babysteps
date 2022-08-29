# hello_world
We are going to create a hello world program to get you familiar with how to create and run programs using the Rust compiler.
We use `cargo` to creat a new project **hello_world**. `Cargo` is Rust build tool, dependancy manager and build system.
To begin, go to your terminal or command prompt and create a new directory called `hello_world` using this command.
```zsh
cargo new hello_world
```
It will create a new directory called **hello_world**, it will also create a new folder and a file called `src/main.rs` and `cargo.toml` inside the **hello_world** directory.
```zsh
❯ cargo new hello_world
     Created binary (application) `hello_world` package
```
- `cargo.toml` is a configuration file that contains all the information needed to compile the program.
- `src/main.rs` is the file where the main function is defined.

Inside the `src/main.rs` file we will write the code that will be executed when the program is run.
```rust
fn main() {
    println!("Hello, world!");
}
```
- `fn` is the function keyword.
- `main` is the name of the function.
- `()` is the return type of the function.
- `println!` is the macro that will print the text inside the parentheses to the console.
- `"Hello, world!"` is the text that will be printed to the console.
- `;` is the semicolon that ends the statement.
- `}` is the closing curly brace that ends the function.

## Running the program
To run the program, we need to compile it.
```zsh
❯ cargo build
     Compiling hello_world v0.1.0 (file:///home/user/hello_world)
     Finished dev [unoptimized + debuginfo] target(s) in 0.01s
     Running `target/debug/hello_world`
     Hello, world!
```
- `cargo build` is the command that will compile the program.
To run the program, we need to run the `hello_world` binary.
- `Hello, world!` is the text that will be printed to the console.
```zsh
❯ cargo run
     Running `target/debug/hello_world`
     Hello, world!
```
- `cargo run` will run the program and print the output to the console.
- `target/debug` is the directory where the binary will be created.
- `target/debug/hello_world` is the name of the binary that will be created when the program or project is compiled.
- `Cargo.lock` is the file that contains all the information about the dependencies of the program or project.

When a program written in Rust, its `main` function is executed. If there is no `main` function, then it isn’t a complete program; it may be a library, though.