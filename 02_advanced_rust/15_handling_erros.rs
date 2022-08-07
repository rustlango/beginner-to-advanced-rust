// lecture notes on Handling Errors

// Error handling
// Error types: 

// recoverable: program may continue with some unwanted side-effect. We can 
// recover from an erroneous state by using the Option enum (see before) 
// and the Result enum (see in this lesson)
// non-recoverable: failure, program shuts down. Also called panicking. 

// Throwing errors
// panic! stops execution of the program and displays its argument as 
// an error message and optionally a backtrace 
// (in other languages: stack trace).

fn erroneous_call() {
    panic!("Error description");
}

fn main() {
    erroneous_call();
}

// cargo run: no backtrace

// RUST_BACKTRACE=1 cargo run: set backtrace to 1 to see the trace information.

// panic! should hardly ever be used in Rust programs, and should only be used 
// for non-recoverable errors.

// Reminder: Option enum 
// When is an option used? 

// to represent an optional value that either exists or not
// a key does not unlock a value in a data structure (e.g. struct or vector)
// Generally, if you are tempted to use null or undefined in JavaScript, the 
// Rust solution is the Option enum.

fn print_value(v: &Vec<i32>, i: usize) -> () {
  match v.get(i) {
    Some(num) => { 
      println!("Value: {}", num);
    } 
    None => {
      println!("None");
    }
  }  
}


fn main() {
  let data = vec![1];
  
  print_value(&data, 0);
  print_value(&data, 1);
}

// The option type can be unwrapped using the unwrap() method called on an 
// Option.
// If the optional value exists, unwrap() returns the value. If the value does 
// not exist, the program panics.

fn print_value(v: &Vec<i32>, i: usize) -> () {
  println!("Value: {}", v.get(i).unwrap()); 
}

fn main() {
  let data = vec![1];
  
  print_value(&data, 0);
  // print_value(&data, 1); // uncomment this line to see the program panic
}

// The ? operator can be applied on an Option. If there is a value, the ? operator
//  gives you the same result as calling unwrap(). However, if there is no value, 
//  the function returns None. As a result, when using ? on an Option in a 
//  function, the program has to return an Option.

// Example:

fn print_and_return_value(v: &Vec<i32>, i: usize) -> Option<i32> {
  println!("Calling print_and_return_value({:?}, {})", v, i);
  let value = v.get(i)?;
  println!("Inside: {}", value); 
  return Some(*value);
}


fn main() {
  let data = vec![1];
  
  let option1 = print_and_return_value(&data, 0);
  println!("Outside: {:?}", option1); 
  let option2 = print_and_return_value(&data, 1);
  println!("Outside: {:?}", option2); 
}

// The Result enum
// Represents a recoverable error, where a potential erroneous state can be handled in e.g. a match statement.

// Ok represents the success case with a ValueType.

// Err represents the error case with a specific Error.

// Result enums can be matched and each case (Ok, Err) can be handled separately.

enum Result<ValueType, Error> {
    Ok(ValueType),
    Err(Error)
}

// Example: 

fn divide(a: f32, b: f32) -> Result<f32, String> {
  if b == 0.0 {
    return Err("Division by zero".to_string());
  } 
  return Ok(a / b);
}

fn print_result(result: Result<f32, String>) -> () {
  match result {
    Ok(value) => { 
      println!("Result: {}", value);
    } 
    Err(message) => {
      println!("Error: {}", message);
    }
  }  
}

fn main() {
  let num = 5.0;
  let b1 = 2.0;
  let b2 = 0.0;

  print_result( divide(num, b1) ); 
  print_result( divide(num, b2) );
}

// A Result can be transformed to ValueType or panic using the unwrap() 
// method of Result instances:

fn divide(a: f32, b: f32) -> Result<f32, String> {
  if b == 0.0 {
    return Err("Division by zero".to_string());
  } 
  return Ok(a / b);
}

fn main() {
  let num = 5.0;
  let b1 = 2.0;
  let b2 = 0.0;

  println!("Result1: {}", divide(num, b1).unwrap() ); 
    println!("Result2: {}", divide(num, b2).unwrap() ); 
}

// The ? operator can also be applied on Result enums:

fn divide(a: f32, b: f32) -> Result<f32, String> {
  if b == 0.0 {
    return Err("Division by zero".to_string());
  } 
  return Ok(a / b);
}

fn print_result(a: f32, b: f32) -> Result<f32, String> {
  let quotient = divide(a, b)?;
  println!("Result: {}", quotient ); 
  return Ok(quotient);
}

fn main() {
  let num = 5.0;
  let b1 = 2.0;
  let b2 = 0.0;

  let result1 = print_result(num, b1);
  println!("main() - Result1: {:?}", result1 ); 
  let result2 = print_result(num, b2);
  println!("main() - Result2: {:?}", result2 ); 
}