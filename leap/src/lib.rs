pub fn is_leap_year(year: u64) -> bool {
    let by_4 = year % 4 == 0;
    let by_100 = year % 100 == 0;
    let by_400 = year % 400 == 0;

    by_4 && (!by_100 || by_400)
}
