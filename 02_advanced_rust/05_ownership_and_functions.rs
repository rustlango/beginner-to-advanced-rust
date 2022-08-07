// lecture notes on Ownership and Functions

// Functions and ownership
// function arguments have the scope of a function 
// when returning a value in a function, the returned value’s ownership is 
// transferred to the variable holding the return value of the function.
// The below code does not make much practical sense, but it illustrates 
// the concept of ownership perfectly:


// There are problems with this approach. Let’s create a function that 
// doubles a string:


// What happened here?

// We create a mutable String with value “reference”
// We pass this string to a function called double_str 
// Upon passing “reference” to double_str, ownership of reference is 
// transferred to argument s of double_str. The scope of s is within 
// the function body. In main, str gets invalidated as it lost ownership 
// of “reference” 
// A new string “referencereference” is created and returned by double_str 
// Upon exiting double_str, s gets out of scope, and therefore, the 
// memory storing “reference” is freed 
// Upon returning “referencereference” the new owner of this new string 
// is str2 
// Consequence: as str is invalidated, we cannot use its value in main
