// lecture notes on Arrays 
// Rust arrays have a fixed length. 

// Each element in a Rust array has the same type.

// Essentially, arrays are like tuples, where the type of each element is fixed
// arrrays are not resizable and to referecne the elements we use bsquare 
// bracket notation. e.g. arrya_example[0]


fn main() {
    let cheat_code: [u32; 4] = [19, 65, 9, 17];
    let zeros = [0.0; 10];
 
    println!("Array: {:?}", cheat_code);
    println!("First element of the array: {}", cheat_code[0]);
 
    println!("Array(length: {}): {:?}", zeros.len() ,zeros);
 
    let slice = &cheat_code[1..3];
    println!("Slice of cheat_code: {:?} {}", slice, slice.len());
}