// lecture notes on loops Loops – part 2

fn main() {
    // Print the first n positive integers
    let n = 10;
    let mut i = 1;
 
    loop {
        println!("loop: {}", i);  // statements inside the loop
        i = i + 1;                // increment
        if i > n {                // terminal condition
            break;
        }        
    }
 
    println!("in-between loops: {}", i);
    i = 1;
 
    while i <= n {                // terminal condition
        println!("while: {}", i); // statements inside the loop
        i = i + 1;                // increment
    }
 
    println!("in-between loops: {}", i);
 
    for i in 1..=10 {             // increment and terminal condition
        println!("for: {}", i);   // statements inside the loop
    }
 
    let cheat_code = [19, 65, 9, 17];
 
    for i in 0..cheat_code.len() {
        println!("cheat code: {}", cheat_code[i]);
    }
 
    for value in cheat_code {
        println!("cheat code by value: {}", value);
    }
}