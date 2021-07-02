# tetris-rs
This is a rewriting of my cpp tetris. The c++ version was pretty advanced. My goal is to cover as many aspects of the language rust as I can in the official documentation : https://doc.rust-lang.org/book/ .
For the moment, the covered notions are :
 - common types an struct (https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
 - ownership (https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
 - struct, enum and pattern matching (https://doc.rust-lang.org/book/ch05-00-structs.html)
 - modules and library (https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)
 - generic types and traits (https://doc.rust-lang.org/book/ch10-00-generics.html)
 - iterators and closures (https://doc.rust-lang.org/book/ch13-00-functional-features.html)

The c++ version suffers from a lot of problems. So, this tetris in rust uses a MVC design pattern to improve lisibility and unlike the cpp version, there are more abstractions. 
Some ui components are implemented in a library to strengthen the boundary between the model and the view. 
The ui components are not signal based but the menu ui will be.
