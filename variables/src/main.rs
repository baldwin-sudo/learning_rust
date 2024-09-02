fn main() {
    let mut x = 5;
    // if we dont use the mut keyword , we will get an error  .
    // Error : you can't reassign immutable variable .
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
