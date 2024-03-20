// not sure about efficiency though!
pub fn nth(n: u32) -> u32 {
    let mut primes = vec![2, 3];
    let mut v: Vec<u32> = vec![];
    let mut c = if n >= 2 { n - 2 } else { n };
    let mut p = 3;
    'l: loop {
        if n == 0 {
            return 2;
        } else if n == 1 {
            return 3;
        } else {
            p = p + 2;
            for v in primes.iter() {
                if p % v == 0 {
                    continue 'l;
                }
            }
            primes.push(p);
            if c > 0 {
                c -= 1;
            } else {
                return p;
            }
        }
    }
}
