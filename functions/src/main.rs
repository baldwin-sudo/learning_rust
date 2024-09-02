fn main() {
    println!("-Hello, world!");
    let result = add_function(5, [7, 1, 2]);
    println!("{result}");
    let number: u32 = 9;
    let fibonacci_n = fibonacci(number as i32);
    println!("-fibonacci {number} is  :{fibonacci_n}")
}
fn add_function(x: i32, y: [i32; 3]) -> String {
    let result = format!(
        "value : {} + {:?} = {}  ",
        x,
        y,
        x + y[0] + y[1] + y[y.len() - 1]
    );
    result
}
fn fibonacci(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}
