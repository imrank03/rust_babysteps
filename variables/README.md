# Variables
A variable is like a storage box paired with an associated name which contains data. The associated name is the identifier and the data that goes inside the variable is the value. They are immutable by default, meaning, you cannot reassign value to them.

Rust cares a great deal about what variables are modifiable. Values fall into two types:

- **mutable** the compiler will allow the variable to be written to and read from. Mutable values are denoted with a `mut` keyword.
- **immutable** the compiler will only allow the variable to be read from.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/variable/variable.svg?raw=true">
</p>

To create a variable, use the `let` binding followed by the variable name.

>What is binding? 
>Rust refers to **declarations** as bindings as they bind a name at the time of creation. `let` is a kind of **declaration statement**.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/variable/let.svg?raw=true">
</p>

> Naming Convention, you would write a variable name in a snake_case i.e.,
> 
> - All letters should be lower case.
> - All words should be separated using an underscore ( _ )


To initialize a variable, use the `=` operator. A variable can be initialized by assigning a value to it when it is declared. The value is said to be bound to that variable.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/variable/initialize.svg?raw=true">
</p>

To reassign a variable, write `let` followed by the `mut` keyword and the variable name.

example:
```zsh
let mut x = 5;
x = 6;
```

## Scope of a variable

The scope of a variable refers to the visibility of a variable, or , which parts of a program can access that variable.

It all depends on where this variable is being declared. If it is declared inside any curly braces `{}`, i.e., a block of code, its scope is restricted within the braces, otherwise the scope is global.

## Types of Variables
There are two types of variables in terms of scope.

**Local Variable**

A variable that is within a block of code, `{ }`, that cannot be accessed outside that block is a local variable. After the closing curly brace, `}` , the variable is freed and memory for the variable is deallocated.

**Global Variable**

The variables that are declared outside any block of code and can be accessed within any subsequent blocks are known as global variables.

The variables declared using the `const` keyword can be declared in local as well as global scope.
```zsh
fn main() {
	let outer_variable = 112;
	{
		let inner_variable = 213;
		println!("block variable inner: {}",inner_variable);
		println!("block variable outer: {}",outer_variable);
	}
	println!("inner variable: {}", inner_variable); // use of inner_variable outside scope
}
```
> Note: The following code gives an error, ❌, since the variable created inside the inner block of code has been accessed outside its scope.

To fix this error, the variable declaration can be moved outside the inner block of code. That way, the scope of the variable spans the entire main() function.

```zsh
fn main() {
    let outer_variable = 112;
    let inner_variable = 213;
    { // start of code block
        println!("block variable inner: {}", inner_variable);
        println!("block variable outer: {}", outer_variable); 
    } // end of code block
    println!("inner variable: {}", inner_variable);
}
```

## Shadowing
Variable shadowing is a technique in which a variable declared within a certain scope has the same name as a variable declared in an outer scope. This is also known as masking. This outer variable is said to be shadowed by the inner variable, while the inner variable is said to mask the outer variable.

The following code explains the concept.

```zsh
fn main() {
    let outer_variable = 112;
    { // start of code block
        let inner_variable = 214;
        println!("block variable: {}", inner_variable);
        let outer_variable = 117;
        println!("block variable outer: {}", outer_variable);
    } // end of code block
    println!("outer variable: {}", outer_variable);
}
```

# Constant Variable

**What Are Constant Variables?**

Constant variables are ones that are declared constant throughout the program scope, meaning, their value cannot be modified. They can be defined in global and local scope.

**Syntax**

They are declared using the `const` keyword followed by the name of the variable, colon (`:`), and then the data type of the variable.

![Screenshot 2022-08-19 at 3.43.21 AM.png](https://s3-us-west-2.amazonaws.com/secure.notion-static.com/e6dd9b35-2c58-4a66-a7f9-f76751ac18f5/Screenshot_2022-08-19_at_3.43.21_AM.png)

> Naming Convention: By convention, you write a constant variable name in a SCREAMING_SNAKE_CASE, i.e.,
> 
> - All letters should be UPPER case.
> - All words should be separated using an underscore ( _ ).

**Example**

The following example defines two `const` variables:

- `ID_1` in **global scope**
- `ID_2` in **local scope**

```rust
const ID_1: i32 = 4; // define a global constant variable

fn main() {
	const ID_2: u32 = 3; // define a local constant variable
	println!("ID:{}", ID_1); // print the global constant variable
	println!("ID:{}", ID_2); // print the local constant variable
}
```

### Difference Between const and let Variables**

There are many differences between `const` and `let` variables.

**Declaration**

- Constant variables are declared using the `const` keyword unlike `let` variables.

**Scope**

- `const` variables are declared in global and local scope unlike `let` variables that are declared only in the local scope.

**Mutability**

- `const` variable cannot be mutable unlike `let` which can be made mutable using `mut` keyword.

**Data Type**

- Unlike `let` variables, it is mandatory to define the data type of `const` variables.

**Set Value at Run-time**

- The value of `const` variable can only be set before running the program whereas the `let` variable can store the result at runtime.

**Shadowing**

- Unlike `let` variables, `const` variables cannot be shadowed.

## Running the program
To run the program, type `cargo run` in the terminal.

```zsh
❯ cargo run  
   Compiling variables v0.1.0 (/Users/imrankhaleelsab/Imran/Boschspace/rust-workshop/rustgrow/variables)
warning: unused variable: `z`
  --> src/main.rs:10:9
   |
10 |     let z: i32; // Uninitialized but also unused, only a Warning!
   |         ^ help: if this is intentional, prefix it with an underscore: `_z`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `variables` (bin "variables") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.56s
     Running `target/debug/variables`
The value of x is: 10
The value of y is 3.14159
The value of z is: 10
The value of z is: 20
```