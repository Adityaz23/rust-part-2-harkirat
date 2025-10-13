// The option enum let you return either some value or none value.
// They are used for handling null values in rust, and for the error handling. (both of them are done by the option and result enum).

// 1. Option enum -> Help you to return null,nil,none values.
// Q. Write a function that returns the index of the first "a" in the string ->

pub fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}

pub fn find_capital_chars(a: String) -> Option<i32>{
    for (index,character) in a.chars().enumerate(){
        if character == 'A'{
            return Some(index as i32);
    } 
}
    return None;
}

// now the result -> 
// The result enum let's ypu return only two values which are OK and Err value , the result enum is how you do error handling in rust.
// You use the result enum when you know there might be the chance of having error in this particular line of code or in this function that it will not work correctly.

// Q. Write a function that reads the content of the file.
// for reading the value from the other places in the rust we need to use the fs library -> use std::fs and the function read_to_string.
use std::fs;

pub fn read_content(){
    let greeting_file = fs::read_to_string("hello.txt");
    match &greeting_file{
        Ok(file_content)=> println!("The content of the file is: {}", file_content),
        Err(error) => println!("Failed to read the file :{}", error)
    }
    // println!("{:?}",greeting_file);
}

// it just simply means to return the type of result that is either error or the string value which is given in the OK or else the error in the Err as the string which is called in the Result enum , OK and Err are the two generics of the Result enum.
// read_to_string => This is the function which is used to read the sdtring and the text of the file.

pub fn get_string(){
    let greet = fs::read_to_string("test/adi.txt");
    match &greet{
        Ok(file_content2)=> println!("The content of the file is : {}", file_content2),
        Err(error)=> println!("failed to read the file which is adi.txt: {}", error)
    } ;
}