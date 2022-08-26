//Variable example
fn main() {
    let x = 10; // x is of type i32, i is an integer type, 32 bits, signed (implictly declared as i32)
    println!("The value of x is: {}", x); // {} is a placeholder for the value of x

    // rust can also be explicit about the type
    let y: f64 = 3.14159;
    println!("The value of y is {}", y); // The value of y is 3.14159

    let z: i32; // Uninitialized but also unused, only a Warning!
    // let z = 10; // uncomment this line to see the error message
    let mut z = 10; // comment this line to see the error message
    println!("The value of z is: {}", z); // The value of z is: 10

    z = 20; // z is now of type i32, can be reassigned because it is mutable (mutable variables are declared with the keyword mut)
            // if line number 11 uncommented and line 12 commented, the program throws an error
    println!("The value of z is: {}", z); // The value of z is: 20
}
