// What is memory management ?
// Memory management is the process of allocating and deallocating memory for variables and data structures in a program.
// Whenever you run a rust program with cargo run the program ends up taking some space in your RAM. This memory is really small.
// The values which can't be increased or decreased in size is known as stack memory.
// Q. When to store the data in the heap memory?
// A. When the size of the data is not known at compile time or when the size of the data is very large.
// Stack memeory management -> Static, allocated at compile time , smaller in size, faster , used for small fixeed size variables and function call information. The things which are stored in the stack are -> numbers,booleans, arrays of fixed size, structs, and references.
// Heap memery management -> Dynamic, allocated at runtime , larger in size , slower due to the dynamic allocation and deallocation , used for dynamic and large data strucutures like vectors , and more. The things which are stored in the heap are -> Strings, vectors, boxes and hashmaps and larged sized arrays and structs.

pub fn memo(){
    let mut name = String::from("Hello Aditya"); // this is 32 bits in the stack but the whole value is not stroed in the stack cause you do not know how long the string is going to be.
    println!("{}",name); // the string is Hello but if you want to add some new values you need to add the mut variable.
    name.pop();
    println!("{}",name);
    name.push('a');
    println!("{}",name);
    name.push_str(" Diksha");
    println!("{}",name);

    // If we try to do the same with the numbers it will be stored in the stack.
    let mut a = 12;
    a = 32; // this will give us the error until the declaration of the variables is not mutable.
    println!("{}",a);
}