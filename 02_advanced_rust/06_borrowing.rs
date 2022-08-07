// lecture notes on borrowing

// Borrowing
// We might not want ownership transfer to happen, as in double_str, we are only using the value of s without owning it:


// The & operator only takes the reference of str. Check the figure in 
// https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#references-and-borrowing 
// to understand how it is represented in memory. In this case: 

// when &str is passed to double_str, the value “reference” is still owned by str 
// s is just a reference, therefore, once s gets out of scope, we do not need to free anything in memory, 
// as s does not own anything
// The act of taking the reference of a value without having ownership of it is called borrowing. 
// Once the borrowed reference gets out of scope, the owner still owns its value.