pub fn factors(n: u64) -> Vec<u64> {
    let mut res: Vec<u64> = vec![];
    let mut f = 2;
    let mut num = n;
    loop {
        if num % f == 0 {
            res.push(f);
            num = num / f;
        } else {
            if f >= num {
                break;
            }
            f += 1;
        }
    }
    res
}
