use std::time::SystemTime;

pub struct DateTimeNow {
    pub year: u64,
    pub month: String,
    pub day: u64,
    pub hours: u64,
    pub minutes: u64,
    pub secs: u64,
}

impl DateTimeNow {
    pub fn new() -> Self {
        let time_sec_from_unix_epoch;
        
        match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => time_sec_from_unix_epoch = n.as_secs(),
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
        
        let year_now = (time_sec_from_unix_epoch / (86400 * 365)) + 1970;
        let mut number_intercalary_years = 0;
        
        for i in 1970..(year_now - 1) {
            if i % 4 == 0 {
                number_intercalary_years += 1;
            }
        }

        let mut diff_in_secs = time_sec_from_unix_epoch - 
            ((year_now - 1970 - number_intercalary_years) * 365
            + number_intercalary_years * 366) * 86400;
        
        let number_day_of_current_year = diff_in_secs / 86400;
    
        diff_in_secs -= (number_day_of_current_year) * 86400;

        let mut months = [("January", 31), ("February", 28), ("March", 31), ("April", 30),
            ("May", 31), ("June", 30), ("July", 31), ("August", 31), ("September", 30), ("October", 31),
            ("November", 30), ("December", 31)];
    
        if year_now % 4 == 0 {
            months[1] = ("February", 29);
        }

        let month_now: String;
        let mut count_days = 0;
        let day_now;

        let mut i = 0;
        loop {
            let (month, days) = months[i];

            if number_day_of_current_year < count_days + days {
                month_now = month.to_string();
                day_now = number_day_of_current_year - count_days + 1;
                break;
            }

            count_days += days;
            i += 1;
        }

        let hours_now = diff_in_secs / 3600;
        diff_in_secs = diff_in_secs - hours_now * 3600;
        let minutes_now = diff_in_secs / 60;
        let secs_now = diff_in_secs - minutes_now * 60;
    
        
        Self {
            year: year_now,
            month: month_now,
            day: day_now,
            hours: hours_now,
            minutes: minutes_now,
            secs: secs_now,

        }
    }
}
