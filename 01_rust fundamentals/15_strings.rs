// lecture notes on strings Strings
// Strings are character sequences.

// In other languages, using strings seems easy. In Rust, you need a lot more 
// knowledge to understand and handle strings, as Rust doesn’t come with memory 
// trade-off decisions. Therefore, Rust strings will generally be more 
// performant and flexible than strings of other languages.

// There are two types of strings in Rust: 

// &str: string slice (referencing the variable type) / string literal 
// (referencing the value encapsulated by double quotes). 
// This is a (mostly) immutable character sequence.
// may be stored on Stack, Heap, or may be embedded in the code 
// optimized for runtime speed
// String: flexible, mutable string stored on the heap.
// Converting a string literal to a mutable string: 

// to_string method 
// String::from method
// Converting a mutable string to a string literal: 

// & dereferencing in front of the string variable. It doesn’t copy the value, 
// but just references the characters (see borrowing).
fn main() {
    // &str: string slice
    // "": string literal
    let name: &str = "Lemuel";
   
    // String: mutable string
    let location: String = "Gauteng, South AFrica".to_string();
    let title: String = String::from("Blockchain Engineer");
 
    let title_slice = &title;
 
    println!("Hi, my name is {}.", name);
    println!("I live in {}.", location);
    println!("This name is {} characters long.", name.len());
    println!("My title is {}.", title);
    println!("Title as a string slice: {}.", title_slice);
}

// Getting a single character from a string: not that easy, because you have
// to check if the character exists.

// String slicing: [Start…EndPlusOne], like in Python, the second index is 
// not included.

// Important: if you index out from the string, your program will crash 
// If you want to get a slice until the end of the string, use [Start..] 
// without specifying the end.

fn main() {
    let digits = "0123456789".to_string();
 
    // Creating string slices
    let octal_digits = &digits[0..8];
    let binary_digits = &octal_digits[0..2];
 
    println!("{} {} {}", digits, octal_digits, binary_digits);
 
    // 'c' - character (primitive type)
    // "c" - immutable string literal
    // Referencing a single character from a string is harder
    if let Some(four) = &digits.chars().nth(12) {
        println!("Four: {}", four);
    } else {
        println!("Error");
    }   
}

// String concatenation: 

// + only works if the left operand is a String and the right operand is an &str.
// + is a left-associative operator, String + &str; → String.
// format! macro, works like print! or println!, but instead of writing to the 
// console, it returns a value array concatenation of &str values 
// creating an empty string and pushing &str values to it
fn main() {
    println!(
        "String concatenation: {} {} {}",
        "first".to_string() + "second",
        format!("{}{}", "first", "second"),
        ["first", "second"].concat(),
    );
 
    let first = "1".to_string();
    let second = "2".to_string();
    let third = "3".to_string();
    let fourth = "4".to_string();
 
    println!(
        "String concatenation: {}",
        first + &second + &third + &fourth
    );
}
