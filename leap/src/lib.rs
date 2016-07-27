pub fn is_leap_year(year: i32) -> bool {
    let is_factor = |n| year % n == 0;
    is_factor(4) && (!is_factor(100) || is_factor(400))
}
