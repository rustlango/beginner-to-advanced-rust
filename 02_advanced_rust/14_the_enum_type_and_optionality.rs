// lecture notes on The Enum Type and Optionality

Enums 
Match statement:

enum LampState {
    Red,
    Amber,
    Green
}
 
fn print_lamp_state(lamp_value: LampState) -> () {
    match lamp_value {
        LampState::Red => println!("Lamp is red."),
        LampState::Amber => println!("Lamp is amber."),
        LampState::Green => println!("Lamp is green.")
    }
}
 
fn print_optional_lamp_state(optional_lamp_value: Option<LampState>) -> () {
    match optional_lamp_value {
        Some(lamp_value) => print_lamp_state(lamp_value),
        None => println!("Lamp is not working.")
    }
}
 
fn add(a: i32, b: Option<i32>) -> Option<i32> {
    match b {
        Some(value) => return Some(a + value),
        None => return None
    }
}
 
fn main() {
    let lamp_value1: LampState = LampState::Amber;
    let lamp_value2: LampState = LampState::Green;
    let lamp_value3: LampState = LampState::Red;
    let optional_lamp_value: Option<LampState> = None;
    let optional_lamp_value2: Option<LampState> = Some(LampState::Green);
 
    print_lamp_state(lamp_value1);
    print_lamp_state(lamp_value2);
    print_lamp_state(lamp_value3);
    print_optional_lamp_state(optional_lamp_value);
    print_optional_lamp_state(optional_lamp_value2);
 
    let i: i32 = 10;
    let j: Option<i32> = Some(20);
    println!("{:?}", add(i, j));
    println!("{:?}", add(i, None));
}
// If-let:

enum LampState {
    Red,
    Amber,
    Green
}
 
fn print_lamp_state(lamp_value: LampState) -> () {
    match lamp_value {
        LampState::Red => println!("Lamp is red."),
        LampState::Amber => println!("Lamp is amber."),
        LampState::Green => println!("Lamp is green.")
    }
}
 
fn print_optional_lamp_state(optional_lamp_value: Option<LampState>) -> () {
    if let Some(lamp_value) = optional_lamp_value {
        print_lamp_state(lamp_value)
    } else {
        println!("Lamp is not working.")
    }
}
 
fn add(a: i32, b: Option<i32>) -> Option<i32> {
    if let Some(value) = b {
        Some(a + value)
    } else {
        None
    }
}
 
fn main() {
    let lamp_value1: LampState = LampState::Amber;
    let lamp_value2: LampState = LampState::Green;
    let lamp_value3: LampState = LampState::Red;
    let optional_lamp_value: Option<LampState> = None;
    let optional_lamp_value2: Option<LampState> = Some(LampState::Green);
 
    print_lamp_state(lamp_value1);
    print_lamp_state(lamp_value2);
    print_lamp_state(lamp_value3);
    print_optional_lamp_state(optional_lamp_value);
    print_optional_lamp_state(optional_lamp_value2);
 
    let i: i32 = 10;
    let j: Option<i32> = Some(20);
    println!("{:?}", add(i, j));
    println!("{:?}", add(i, None));
}