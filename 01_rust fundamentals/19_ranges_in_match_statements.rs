// lecture notes on Ranges in Match Statements

fn main() {
   
    let name = "Lemuel";
    let name_len = name.len();
 
    // 7. handling ranges with conditionals
    if name_len <= 5 {
        println!("Short");
    } else if name_len >= 6 && name_len < 11  {
        println!("Medium");
    } else {
        println!("Long");
    }
 
    match name_len {
       0..=5  => { println!("Short"); }
       6..=10 => { println!("Medium"); }
       _      => { println!("Long"); }
    }
   
}