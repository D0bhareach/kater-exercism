pub fn is_leap_year(year: u64) -> bool {
    // Keep It Damn Simple Dummy
    // then it is a leap year
    if year % 400 == 0 {
        return true;
    }

    // Else If a year is multiple of 100,
    // then it is not a leap year
    if year % 100 == 0 {
        return false;
    }

    // Else If a year is multiple of 4,
    // then it is a leap year
    if year % 4 == 0 {
        return true;
    }
    return false;
}
