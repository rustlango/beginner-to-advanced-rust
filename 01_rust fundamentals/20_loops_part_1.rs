// lecture notes on Loops – part 1

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
}

// Exercise: why doesn’t the below while loop print the ten positive integers 1, 2, 3, …, 10? 

// Hint: there are two issues with the code.

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
 
    while !(i >= n) {             // terminal condition
        println!("while: {}", i); // statements inside the loop
        i = i + 1;                // increment
    }
}