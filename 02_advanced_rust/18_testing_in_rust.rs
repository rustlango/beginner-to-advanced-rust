// lecture notes on testing in rust

// Create a new lib:

cargo new --lib roman 
cd roman 
code .

// This lib contains a test. Tests are in modules.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// To run your tests, execute

cargo test

// To test some code, add a function: 

pub fn convert(_roman_numeral: &str) {
}

// For now, we are not making use of roman_numeral, so we place an underscore in 
// front of it to indicate we expect the compiler not to warn us about the 
// unused variable.

// To test this convert function, we have to understand that tests is also 
// a module. To import the convert function, we have to use the super 
// keyword inside the tests module to reference the parent. If we want to 
// import everything, we can simply use the asterisk wild card:

use super::*;

// Let’s write a failing test:

pub fn convert(_roman_numeral: &str) -> u32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_should_return_one() {
        assert_eq!(convert("I"), 1);
    }
}

// To make the test pass, change the return value from 0 to 1 and run cargo test.

// Let’s write a second test and write just enough code to make the test pass:

pub fn convert(roman_numeral: &str) -> u32 {
    match roman_numeral {
        "II" => { return 2; }
        _    => { return 1; }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
     // the bellwo test comment is an annotation to note that the next function
    // following it is a test caase that needs to be tested.
    #[test]
    fn it_should_return_one() {
        assert_eq!(convert("I"), 1);
    }
     // the bellwo test comment is an annotation to note that the next function
    // following it is a test caase that needs to be tested.
    #[test]
    fn it_should_return_two() {
        assert_eq!(convert("II"), 2);
    }
}

// Let’s write a third test:

#[cfg(test)]
mod tests {
    use super::*;
     // the bellwo test comment is an annotation to note that the next function
    // following it is a test caase that needs to be tested.
    #[test]
    fn it_should_return_one() {
        assert_eq!(convert("I"), 1);
    }
     // the bellwo test comment is an annotation to note that the next function
    // following it is a test caase that needs to be tested.
    #[test]
    fn it_should_return_two() {
        assert_eq!(convert("II"), 2);
    }
    // the bellwo test comment is an annotation to note that the next function
    // following it is a test caase that needs to be tested.
    #[test]
    fn it_should_handle_additive_digits() {
        // assertion_eq macros check if the left hand side equals 
        // the right hand side. If it does the test passes, if not the test fails
        assert_eq!(convert("VII"), 7);
        assert_eq!(convert("MMMDCCCLXXXVIII"), 3888);
    }
}

// Change the function to make this test pass: 

pub fn convert(roman_numeral: &str) -> u32 {

    let mut sum = 0;
    for ch in roman_numeral.chars() {
        println!("ch {}", ch);
        match ch {
            'I' => { sum += 1; } 
            'V' => { sum += 5; }
            'X' => { sum += 10; } 
            'L' => { sum += 50; }
            'C' => { sum += 100; } 
            'D' => { sum += 500; } 
            'M' => { sum += 1000; } 
            _ => {}
        }
    }

    return sum;
}

// Feel free to finish the Roman numeral converter function and share it 
// in the forum. Use test driven development: 

// Write a failing test
// Write just enough code to make it pass
// Refactor – simplify the code without changing its behavior. This 
// step is guarded by your tests