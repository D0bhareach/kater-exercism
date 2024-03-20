pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }

    let mut steps = 0;
    let mut num: u64 = n;
    while num != 1 {
        if num % 2 == 0 {
            num = num / 2;
            steps += 1;
        } else {
            let t = dbg!(num.overflowing_mul(3));
            if !t.1 {
                let tt = t.0.overflowing_add(1);
                if !tt.1 {
                    num = tt.0;
                    steps += 1;
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
    Some(steps)
}
