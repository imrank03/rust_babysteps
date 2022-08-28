# Scalar
scalar has two types:
- Numeric types (integers and floats)
- Non-numeric types (booleans and characters)

# Numeric Types

## Integers

Variables of Integer data type hold whole number values. There are two subtypes of integer data type in Rust, based on the number of bits occupied by a variable in memory.

**Fixed Size Types**

The fixed integer types have a specific number of bits in their notation. This notation is a combination of a letter and a number. The former denotes the category of the integer, whether it is, unsigned or signed, and the latter denotes the size of an integer, i.e., 8, 16, 32, 64.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/scalar/integer.svg?raw=true">
</p>

Below is the list of fixed length integer types:

- `i8`: The **8-bit signed** integer type.
- `i16`: The **16-bit signed** integer type.
- `i32`: The **32-bit signed** integer type.
- `i64`: The **64-bit signed** integer type.
- `u8`: The **8-bit unsigned** integer type.
- `u16`: The **16-bit unsigned** integer type.
- `u32`: The **32-bit unsigned** integer type.
- `u64`: The **64-bit unsigned** integer type.

**Variable Size Types**

The integer type in which the particular size depends on the underlying machine architecture.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/scalar/variable_size.svg?raw=true">
</p>

- `isize`: The **pointer-sized signed** integer type.
- `usize`: The **pointer-sized unsigned** integer type.

> 💡 Why are there so many types of integers and how do you pick a data type?
> 
> 
> The choice depends on what values a variable is expected to hold. So, a programmer should pick a data type that is not so small that the data is lost. Nor should they pick a data type that is so big that it wastes memory.
> 

**Example**

The code below defines an integer type both explicitly and implicitly:

**Explicit Definition**

The following code explicitly defines the integer variables using the integer type fixed or variable):

```rust
fn main() {
    //explicitly define an integer
    let a:i32 = 24;
    let b:u64 = 23;
    let c:usize = 26;
    let d:isize = 29;
    //print the values
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}
```

**Implicit Definition**

The following code implicitly defines the integer type of the variable by assigning an integer value to the variable.

```rust
fn main() {
    //implicitly define an integer
    let a = 21;
    let b = 1;
    let c = 54;
    let d = 343434;
    //print the variable
    println!("a: {}", a);
    println!("b: {}", b);
    println!("c: {}", c);
    println!("d: {}", d);
}
```

## Floating Point

Floating-point numbers refer to numbers with a fractional part.

The representation of floating-point numbers in a computer’s memory is such that the precision with which a number is stored in memory depends on the number of bits used for storing the variable.

In this respect, there are two subtypes: single-precision `f32` and double-precision `f64` floating-point, with the latter having more bits to store the number.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/scalar/float.svg?raw=true">
</p>

- `f32`: The **32-bit floating point** type.
- `f64`: The **64-bit floating point** type.

**Example**

The code below defines a floating-point number both explicitly and implicitly:

**Explicit Definition**

The following code explicitly defines the float variable using the float type (`f32` or `f64`):

```rust
fn main() {
    //explicitly define a float type
    let f1:f32 = 32.9;
    let f2:f64 = 6789.89;
    println!("f1: {}", f1);
    println!("f2: {}", f2);
}
```

**Implicit Definition**

The following code implicitly defines the float type of the variable by assigning a floating-point value to the variable:

```rust
fn main() {
	//implicitly define a float type
	let pi = 3.14;
	let e = 2.17828;
	println!("pi: {}", pi);
	println!("e: {}", e);
}
```

# Non-Numeric types

## **Boolean**

The boolean variable can take a value either **true or false**.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/scalar/bool.svg?raw=true">
</p>

**Example**

The following code explains how to define a boolean variable in three different ways:

**Explicit Definition**

The following code explicitly defines the variable using the `bool` keyword:

```rust
fn main() {
	//explicitly define a bool
	let is_bool:bool = true;
	println!("explicitly_defined: {}", is_bool);
}
```

**Implicit Definition**

The following code implicitly defines the boolean type of a variable by assigning the value `true` or `false` to the variable.

```rust
fn main() {
	// assign a boolean value
	let a = true;
	let b = false;
	println!("a: {}", a);
	println!("b: {}", b);
}
```

**Result of an Expression**

The result of an expression that evaluates to either true or false (for example a comparison of two values) can be assigned to an implicit boolean variable.

```rust
fn main() {
	// get a value from an expression
	let c = 10 > 2;
	println!("c: {}", c);
}
```

## **Character**

The variable is used to store a single character value, such as a single digit or a single alphabet. The value assigned to a char variable is enclosed in a single quote(`''`) .

> Note: Unlike some other languages, a character in Rust takes up 4 bytes rather than a single byte. It does so because it can store a lot more than just an ASCII value like emojis, Korean, Chinese, and Japanese characters.
> 

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/scalar/char.svg?raw=true">
</p>

**Example**

The code below defines a character both explicitly and implicitly:

**Explicit Definition**

The following code explicitly defines the variable using the `char` keyword:

```rust
fn main() {
	// explicitly define
	let char_1:char = 'e';
	println!("character1: {}", char_1);
}
```

**Implicit Definition**

The following code implicitly defines the character type of the variable by assigning the single value enclosed within single quotes to them.

```rust
fn main() {
	// implicitly define
	let char_2 = 'a';
	let char_3 = 'b';
	println!("character2: {}", char_2);
	println!("character3: {}", char_3);
}
```

## **String**

A string is any sequence of characters enclosed within double quotes (`" "`).

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/scalar/string.svg?raw=true">
</p>

**Example**

The code below defines a string both explicitly and implicitly:

**Explicit Definition**

The following code explicitly defines the variable using the `&str` keyword:

```rust
fn main() {
	// explicitly define
	let str_1:&str = "Rust Programming";
	println!("String 1: {}", str_1);
}
```

**Implicit Definition**

The following code implicitly defines the string type of the variable by assigning the single value enclosed within double quotes to them.

```rust
fn main() {
    // implicitly define
    let str_2 = "Rust Programming";
    println!("String 2: {}", str_2);
}
```