pub fn square_of_sum(n: u32) -> u32 {
    (0..=n).sum::<u32>().pow(2u32)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (n * (n + 1) * (2 * n + 1)) / 6 as u32
}

pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}
