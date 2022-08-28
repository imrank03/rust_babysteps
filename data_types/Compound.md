# Compound
It has two types:
- Arrays
- Tuples

# Array

**What Is an Array?**

An array is a **homogenous sequence of elements**. Being a compound type, it is used when the collection of values of the same type are to be stored in a single variable. In Rust, an array can only be of a fixed length. Like all other languages, each element in the array is assigned an index. By default, the first element is always at index 0.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/compound/array.svg?raw=true">
</p>

> Note: By default, arrays are immutable.
> 

**Define an Array**

To define an array in Rust, we have to define the type and size of the array. To initialize an array, the array elements are enclosed in square brackets `[]`. The following illustration explains the concept:

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/compound/array1.svg?raw=true">
</p>

```rust
#[allow(unused_variables, unused_mut)]

fn main() {
	//define an array of size 4
	let arr:[i32;4] = [1, 2, 3, 4];
	// initialize an array of size 4 with 0
	let arr1 = [0 ; 4];
}
```

- The array `arr` declaration on **line 4** declares an array with elements 1,2,3,4.
- The array `arr1` declaration on **line 6** implicitly determines the data type (integer) from the value 0 and 4 is the size of the array. So, this becomes an array consisting of 4 zeros.

**Access an Element of an Array**

Any value of the array can be accessed by writing the array name followed by the index number enclosed within square brackets `[ ]`.

```rust
fn main() {
	//define an array of size 4
	let arr:[i32;4] = [1, 2, 3, 4];
	//print the first element of array
	println!("The first value of array is {}", arr[0]);
	// initialize an array of size 4 with 0
	let arr1 = [0; 4];
	//print the first element of array
	println!("The first value of array is {}", arr1[0]);
}
```

**How to Make an Array Mutable?**

Just like a variable becomes mutable by adding the `mut` keyword after let, the same goes for an array.

```rust
fn main() {
	//define a mutable array of size 4
	let mut arr:[i32;4] = [1, 2, 3, 4];
	println!("The value of array at index 1: {}", arr[1]);
	arr[1] = 9;
	println!("The value of array at index 1: {}", arr[1]);
}
```

**Print the Array**

The whole array can be traversed using a *loop* or the *debug trait*.

> The arrays elements can be traversed using loops.
> 

```rust
fn main() {
	//define an array of size 4
	let arr:[i32;4] = [1, 2, 3, 4];
	//Using debug trait
	println!("\nPrint using a debug trait");
	println!("Array: {:?}", arr);
}
```

**Get the Length of the Array**

To access the length of the array, use the built-in function `len`.

```rust
fn main() {
	//define an array of size 4
	let arr:[i32;4] = [1, 2, 3, 4];
	// print the length of array
	println!("Length of array: {}", arr.len());
}
```

# Tuples

**What are Tuples?**

Tuples are **heterogeneous sequences of elements**, meaning, each element in a tuple can have a different data type. Just like arrays, tuples are of a fixed length.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/compound/tuple.svg?raw=true">
</p>

**Define a Tuple**

A tuple can be defined by writing `let` followed by the name of the tuple and then enclosing the values within the parenthesis.

**Syntax 1**

The syntax below defines a tuple without specifying the type. However, the compiler can infer the type.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/compound/tuple1.svg?raw=true">
</p>

**Syntax 2**

The syntax below defines a tuple by specifying the type.

<p align="center">
<img src="https://github.com/imrank03/rustgrow_images/blob/main/data_types/compound/tuple2.svg?raw=true">
</p>

**Example**

The following illustration explains the concept:

```rust
#[allow(unused_variables, unused_mut)]
fn main() {
	//define a tuple
	let person_data = ("Alex", 48, "35kg", "6ft");
	// define a tuple with type annotated
	let person_data : (&str, i32, &str, &str) = ("Alex", 48, "35kg", "6ft");
}
```

**Access the Value of the Tuple**

- Unlike array which uses [] for accessing an element, the value of the tuple can be accessed using the dot operator (`.`).

```
    tuplename.indexvalue
```

- To get the individual values out of a tuple, we can use pattern matching to destructure a tuple value, like this:

```rust
    let person_data = ("Alex", 48, "35kg", "6ft");
    let (w, x, y, z) = person_data;
```

```rust
fn main() {
	//define a tuple
	let person_data = ("Alex", 48, "35kg", "6ft");
	// access value of a tuple
	println!("The value of the tuple at index 0 and index 1 are {} {}",person_data.0,person_data.1);
	//define a tuple
	let person_data = ("Alex", 48, "35kg", "6ft");
	// get individual values out of tuple
	let (w ,x, y, z) = person_data;
	//print values
	println!("Name : {}", w);
	println!("Age : {}", x);
	println!("Weight : {}", y);
	println!("Height : {}", z);
}
```

**How to Make a Tuple Mutable?**

Just like a variable becomes mutable by adding the `mut` keyword after `let`, the same goes for a tuple.

```rust
fn main() {
	//define a tuple
	let mut person_data = ("Alex", 48, "35kg", "6ft");
	//print the value of tuple
	println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
	//modify the value at index 0
	person_data.0 = "John";
	//print the modified value
	println!("The value of the tuple at index 0 and index 1 are {} {}", person_data.0, person_data.1);
}
```

**Print the Tuple**

The whole tuple can be traversed using the *debug trait*.

```rust
fn main() {
	//define a tuple
	let person_data = ("Alex", 48, "35kg", "6ft");
	//print the value of tuple
	println!("Tuple - Person Data : {:?}", person_data);
}
```