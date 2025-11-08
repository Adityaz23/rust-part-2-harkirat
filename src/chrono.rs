use chrono::{Local,Utc,Duration};

pub fn chrono_data_time(){
    let local = Local::now();
    let utc = Utc::now();
    println!("{},{}",local,utc)
}
pub fn chrono_d_t(days:i64,hours:i64,minutes:i64){
    let dati = Utc::now();
    let future = dati + Duration::days(days)+Duration::hours(hours)+Duration::minutes(minutes);
    println!("The future date and time after adding {} days, {} hours and {} minutes are: {}",days,hours,minutes,future)
}