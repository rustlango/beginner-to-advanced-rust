// lecture notes on if-else
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
}