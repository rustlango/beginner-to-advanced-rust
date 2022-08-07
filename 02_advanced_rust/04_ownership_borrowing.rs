// lecture notes on Ownership Borrowing

// Ownership rules 
// Owner of a value: block-scoped variable 
// During execution, once a variable gets out of scope, its value is dropped 
// A value can be owned by one variable, not more
// Value and reference types
// In many programming languages, we have value and reference types. Value types: 

// Numeric values (signed, unsigned integer, float), 
// booleans, 
// characters, 
// tuples containing only value types in any depth recursively. 
// These types are said to implement the Copy trait. The Copy trait ensures that
// the value of these variables are copied upon assignment to a new memory 
// location on the stack.

// When a variable of these types is used in the right value, their value is 
// substituted and the left value of the expression is given a new memory address 
// (num2 contains the value 5 at a different memory address below):


// Result is 6 5. 

// Reference types: e.g. Strings. See the documentation for a full visualization 
// here: https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#ways-variables-and-data-interact-move. 

// Once we assign a string to a new string variable, the original string variable
// gets invalidated. Therefore, we cannot even use str to print the value it holds. 
// This is needed so that once str gets out of scope, we don’t free memory 
// referenced by str, as new_str now owns that area in the memory.



// As only new_str is valid, and new_str owns the “reference” string, we only 
// need to watch when new_str gets out of scope and then free the memory allocated for it.

// If you want to copy strings by value, clone them. Cloning is more time consuming though:

