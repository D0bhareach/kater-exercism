pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64");
    }
    let mut res: u64 = 0;
    (0..s).for_each(|i| if i == 0 { res = 1 } else { res = res << 1 });
    res
}

pub fn total() -> u64 {
    (1..65_i32).map(|i| square(i.try_into().unwrap())).sum()
}
