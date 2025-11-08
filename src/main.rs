use std::cmp::max;
use std::cmp::min;

use crate::enu::move_player;
use crate::option::{external_keyboard_test, find_capital_chars, find_first_a, get_string, read_content};
use crate::packages::date_time;
use crate::struc::User;
mod enu;
mod option;
mod struc;
mod packages;

// # Now we are practising some basic things which will recap us the part-1 of the harkirat rust series ->
// Q. Write a function is_even that taked number as input and returns true if the number is even else false.

fn is_even(num: u64) -> bool {
    if num % 2 == 0 {
        return true;
    } else {
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

// Q. Write a function fib to find the fibonacci number of a given number n it takes as input.

// this is the recursive version of the fibonacci series.

// fn fib(fb: u64)-> u64{
//     if fb==0{
//         return 0;
//     }
//     else if fb == 1 {
//         return 1;
//     }
//     else {
//         return fib(fb-1)+fib(fb-2);
//     }
// }

// another way -> This is iterative version of the fibonacci series. This is more efficient than the recursive one.because it time complexity is O(n) and space complexity is O(1).
fn fib(num: u64) -> u64 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }
    if num == 1 {
        return second;
    }
    for _ in 1..num {
        let temp = second;
        second = first + second;
        first = temp;
    }
    return second;
}
fn main() {
    println!("Hello, world!");
    /*
       In this rust file we are doing recap of the harkirat singh course of rust part-1 and now after that we will do the part-2 of the course and then in the 3rd part we will do the solana wokshops and then after the recap of the part-1 we will make 2 projects with it like cli and http connection based project.

       Recalling Part-1 of Harkirat Singh's Rust Course:
    */
    let a = 12;
    let b = 32;
    let c = a + b;
    println!("The sum of a and b is: {}", c);
    // outermain();
    println!("The number is: {}", is_even(23244234));
    // println!("The fibonacci number is: {}", fib(20))
    // let x = 23;
    // println!("{}",x);
    println!("The fib number is: {}", fib(28));
    let my_string = String::from("Harkirat rust series part-2");
    let length = get_string_length(&my_string);
    println!("The length of the the stirng is: {}", length);
    // struc::struc("Aditya");
    // we can also do one thing which is that we can impl the area function in the struct itself. in the struct.rs file.
    // rust does not know how to print the struct that's why they will print the address of the struct or the memory location of the struct.
    let user1 = struc::create_user();
    println!(
        "The user details are: name:{}, age:{}, email:{}, sign_in:{}",
        user1.name, user1.age, user1.email, user1.sign_in
    );
 
    let m = max(23, 12);
    println!("The maximum number is: {}",m);

    let n = min(43, 12);
    println!("{}",n);

    let user = User {
        name: String::from("Halo"),
        age: 22,
        email: String::from("adi"),
        sign_in: false,
    };
    println!("User is : {}", user.age);
    let length = struc::rect_len();
    let area = length.height * length.widht;
    println!("The area of rectangle is: {}", area);

    let squ_len = struc::squ_len();
    let area_square = squ_len.h * squ_len.w;
    println!("The are of square is: {}", area_square);

    // enu() // calling the enu module from the enu.rs file.

    let player = enu::Direction::Up;
    // move_player(player);
    let player2 = enu::Direction::Down;
    let player3 = enu::Direction::Left;
    let player4 = enu::Direction::Right;

    println!(
        "Direction of the players are: {:?}, {:?}, {:?}, {:?}",
        player, player2, player3, player4
    );
    let player_move = enu::Direction::Up;
    move_player(player_move);

    // we do not need to store the enums in the variable we can directly call them in the function.
    enu::shape(enu::Shape::Circle(3.65));
    enu::shape(enu::Shape::Square(38));
    enu::shape(enu::Shape::Rectangle(3, 2));

    // Option enum function call ->
    let my_string = String::from("Aditya");
    match find_first_a(my_string) {
        Some(index) => println!("The index of first a is: {}", index),
        None => println!("The letter a is not found in the string"),
    }

    let my_capital = String::from("deekshA");
    match find_capital_chars(my_capital) {
        Some(index) => println!("The index of the capital A is: {}", index),
        None => println!("The capital A is not found in the stirng"),
    }

    // Result enum function call ->
    read_content();
    get_string();
    external_keyboard_test();

    // packages function right here ->
    date_time();
}

/*
fn fib(num:u64)->u64{
    let mut first =0;
    let second = 1;
    if (num==0){
    return first;}
}
    if (num==1){
    return second;

    for i in 1..num{
    let temp = second;
    second = first+second;
    first = temp;
    }
*/

/*
    Q. Write  function get_String_length that takes a string as input and returns the length of the string.
    let my_string = String::from ("Hello Okay");
    let length = get_String_length(&my_String);
    println!("Length {}",length);

    fn get_String_length(s &str)->usize{
    s.chars().count() or return s..chars()count();
    }
*/

fn get_string_length(s: &str) -> usize {
    s.chars().count()
}
