// lecture notes If-else and match with branch values
fn main() {
   
    let name = "Lemuel";
    let name_len = name.len();
 
    // 1. simple if-else
    if name_len % 2 == 0 {
        println!("Even");
    } else {
        println!("Odd");
    }
 
    // 2. multiple if-else branches with numbers
    if name_len == 4 {
        println!("Four");
    } else if name_len == 5 {
        println!("Five");
    } else if name_len == 6 {
        println!("Six");
    } else {
        println!("Different");
    }
 
    // 3. multiple if-else branches with boolean expressions
    let first = false;
    let second = true;
 
    if first && second {
        println!("Both are true");
    } else if first || second {
        println!("At least one of them is true");
    } else {
        println!("None of them");
    }
 
    // 4. match statement
    match name_len {
        4 => { println!("Four"); }
        5 => { println!("Five"); }
        6 => { println!("Six"); }
        _ => { println!("Different"); }
    }
 
    // 5. boolean match
    match first {
        true => { println!("True"); }
        false => { println!("False"); }
    }
 
    // 6. if-else as values
    let message = if name_len == 4 { "Four" } else { "Not Four" };
 
    let message2 = if name_len == 4 {
        "Four"
    } else if name_len == 5 {
        "Five"
    } else if name_len == 6 {
        "Six"
    } else {
        "Different"
    };
 
    let message3 = match name_len {
        4 => { "Four" }
        5 => { "Five" }
        6 => { "Six" }
        _ => { "Different" }
    };
 
    println!("{} {} {}", message, message2, message3);
   
}