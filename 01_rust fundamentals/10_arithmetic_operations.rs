// lecture notes Arithmetic operations

// Make sure you have exactly matching types on the operands.

    let first = 7.0;
    let second = 4.0;
    // integer division rounds off to the nearest whole number.
    // 7/4 as integers equals 1 not 1.75. You have change the types to 
    // floating points
    println!(
        "{} {} {} {} {}",
        first+second,
        first-second,
        first*second,
        first/second,
        first%second
    );

// Both first and second have to have the exact same type (here they are implied 
// as f64). If you had an f32 and an f64 variable, arithmetic operations would not
// work.

// If you used integers (here i32), the result of the integer division is an 
// integer. Seven divided by four equals one, and the remainder is 3. 
// You can get the remainder using the modulus (%) operator.

    let first = 7;
    let second = 4;
 
    println!(
        "{} {} {} {} {}",
        first+second,
        first-second,
        first*second,
        first/second,
        first%second
    );