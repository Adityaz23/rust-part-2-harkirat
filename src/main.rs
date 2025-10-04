fn main() {
    println!("Hello, world!");
    /*
        In this rust file we are doing recap of the harkirat singh course of rust part-1 and now after that we will do the part-2 of the course and then in the 3rd part we will do the solana wokshops and then after the recap of the part-1 we will make 2 projects with it like cli and http connection based project.

        Recalling Part-1 of Harkirat Singh's Rust Course:
     */
    let a = 12;
    let b = 32;
    let c = a+b;
    println!("The sum of a and b is: {}",c);
    // outermain();
    println!("The number is: {}",is_even(2324));
}
// # Now we are practising some basic things which will recap us the part-1 of the harkirat rust series ->
// Q. Write a function is_even that taked number as input and returns true if the number is even else false.

fn is_even(num: i64)-> bool{
    if num % 2 == 0{
        return true;
    }
    else {
        return false;
    }
}
// another way to solve->
// fn is_even(){
//     let num = 3;
//     println!("The user enterd the number to check if the number is even or odd is: {}",num);
//     if num % 2==0{
//         println!("The number is even!");
//     }
//     else {
//         println!("The number is not even, hence it is odd!");
//     }
// }
