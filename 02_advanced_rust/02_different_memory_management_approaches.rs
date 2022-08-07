// lecture notes on Different Memory Management Approaches

// Ownership, references, borrowing
// Rust’s concept of memory management.

// Memory management options
// memory allocation and manually freeing memory: precise, fast, but 
// potentially dangerous; tough to implement.
// garbage collection: slow, limited control over memory management, 
// but easier to implement ownership: fast, based on compile-time rules. 
// Variables own values (memory allocation on initialization), and once 
// values get out of scope, the system frees their allocated memory. 
// Best of two worlds.
