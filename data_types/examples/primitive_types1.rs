//! primitive_types1
//! - Integers
//!     - unsigned (u8, u16, u32, u64, u128)
//!     - signed (i8, i16, i32, i64, i128)
//!     - Pointer sized integers (usize, isize)
//! Note: If we don't explicitly assign a type to a variable, then the compiler will infer one for us.

fn main() {
    // type implicitly inferred to be i32
    let x = 5;
    println!("The value of x is: {}", x);
    
    // type explicitly specified
    let y: i32 = 10;
    println!("The value of y is: {}", y);

    let sum = x + y;
    println!("The sum of x and y is: {}", sum);
}
