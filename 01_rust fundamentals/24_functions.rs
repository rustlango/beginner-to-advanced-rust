// lecture notes on functions

fn sum(a: i32, b: i32) -> i32 {
    // statements
    println!("first statement");
    println!("second statement");
    a + b
}
 
fn max(a: i32, b: i32) -> i32 {
    if a >= b {
        a
    } else {
        b
    }
}
 
fn print_max(a: i32, b: i32) {
    println!("max({}, {}) = {}", a, b, max(a, b));
}
 
/*
    fact(0) = 1
    fact(N) = N * fact(N - 1)
 
    fact(5) = 5 * 4 * 3 * 2 * 1 * 1
*/
fn fact(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * fact(n - 1)
    }
}
 
 
 
fn main() {
    let a = 2;
    let b = 5;
    println!("{} + {} = {}", a, b, sum(a, b));
    print_max(a, b);
    print_max(b, a);
    println!("fact(5) = {}", fact(5));
    println!("fact(0) = {}", fact(0));
    println!("fact(10) = {}", fact(10));    
}