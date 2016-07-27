extern crate chrono;
use chrono::*;

pub fn after(start_date: DateTime<UTC>) -> DateTime<UTC> {
    let ten: i64 = 10;
    start_date + Duration::seconds(ten.pow(9))
}
