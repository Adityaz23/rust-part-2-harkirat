// They are similar to the npm packages just like react and node has.
// You do not need to bring the external modules to your project like we use to do in the react or nextjs everything is prebuilt in the rust language.
// To add external dependecies we can go to the cargo.toml file and there is a dependencies section where you can add external dependecies.
// The technical thing is that you can add the external crates they are called crates.
// We will use the chrono which is the external crate for date and time.
// To add it you need to run cargo add chrono.

use chrono::{Local,Utc};
pub fn date_time(){
    // getting the current time.
    let time = Utc::now();
    println!("The date and time in the UTC is: {}", time);
    // formattng the date and time.
    let formated_time = time.format("%Y-%m-%d %H:%M:%S");
    println!("The formatted time is: {}",formated_time);
    // getting the local time.
    let local_time = Local::now();
    println!("This is the local time: {}",local_time);
}