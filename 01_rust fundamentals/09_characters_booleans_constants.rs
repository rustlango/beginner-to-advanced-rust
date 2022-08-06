// lecture notes Characters
fn main() {
    // use signle quotes for char so that the impliead type is char 
    let ch1 = 'X';
    // backslash means a special character will be initialised
    let ch2 = '\u{2603}';
  
    println!("{}, {}", ch1, ch2);
}
Booleans
fn main() {
    let on: bool = true;
    let off: bool = false; 
  
    println!("{}, {}", on, off);
}
// Immutable and mutable variables
// Scalar variables are immutable by default.
// below code will not compile because variables need to be made mutable

fn main() {
    let is_lamp_on: bool = false;
  
    println!("{}", is_lamp_on);

    is_lamp_on = true;

    println!("{}", is_lamp_on);
}
// Make the variable mutable by adding mut in front of the variable name.

fn main() {
    let mut is_lamp_on: bool = false;
 
    println!("{}", is_lamp_on);
 
    is_lamp_on = true;
 
    println!("{}", is_lamp_on);
}
// Alternative: using shadowing. Variables can be redeclared using let.

fn main() {
    let is_lamp_on: bool = false;
 
    println!("{}", is_lamp_on);
 
    let is_lamp_on = !is_lamp_on;
 
    println!("{}", is_lamp_on);
}

// Constants

// Constants are not declared using let, but const. Differences: 

// const keyword 
// type has to be declared 
// the value of a constant cannot be computed during runtime
// mut cannot be used with const
// naming conventions: ALL_CAPS_WITH_UNDERSCORE
// const are used for frquently reference values likes base urls or 
// smart contract address