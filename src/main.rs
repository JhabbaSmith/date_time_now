use time_now::DateTimeNow;

fn main() {
    
    let dt1 = DateTimeNow::new();
    
    println!("date now is:\t {}-{}-{}", dt1.day, dt1.month, dt1.year);
    println!("time now is:\t {}:{}:{}", dt1.hours, dt1.minutes, dt1.secs);

}

