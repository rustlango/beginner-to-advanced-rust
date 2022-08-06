// research on stack and heap

// Please use Google to answer the following questions:

// 1. When it comes to memory management, what does it mean that our 
// data are stored on the stack? What are the advantages and disadvantages 
// of using the stack?

// The stack is for fixed length data types - meaning that the size of the 
// variable is known at compile time. Data stored on the stack has the 
// advantage of better retrievability because the size is of the data is small.  
// A local variable can be allocated to the stack memory during the function 
// but when a it is returned at the end of the function program then it will 
// be automatically deallocate which is an advantage of the stack model. 

// 2. When it comes to memory management, what does it mean that our data 
// are stored on the heap? What are the advantages and disadvantages of using 
// the heap?**
// The heap sector of the memory model of rust is dynamic data that is not known 
// at compile time. For this reason the heap has a bigger memory capacity 
// (advantage over stack), however, the disadvantage of this is that the 
// data takes longer to be retrieved and the data that is referenced 
// has to removed/dereferenced manually after the lifetime of the program 