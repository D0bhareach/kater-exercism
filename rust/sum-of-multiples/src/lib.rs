fn get_multiples(limit: u32, f: u32) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    (f..limit).for_each(|n| {
        if n % f == 0 {
            res.push(n);
        }
    });
    res
}

fn get_one_factor(limit: u32, n: u32) -> u32 {
    return if n == 0 {
        0
    } else {
        if limit < n {
            0
        } else {
            let res = get_multiples(limit, n);
            res.iter().sum()
        }
    };
}

fn get_sum(mut vct: Vec<u32>) -> u32 {
    vct.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vct.dedup();
    vct.iter().sum()
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // get list of primes
    let mut res: Vec<u32> = vec![];
    if factors.len() == 0 {
        return 0;
    }

    if factors.len() == 1 {
        let n = factors[0];
        return get_one_factor(limit, n);
    }
    // can remove zero now
    let mut v = factors[..].to_vec();
    v = v.iter().filter(|n| *n != &0).copied().collect();

    // start all over again. Handele special cases again.
    if v.len() == 0 {
        return 0;
    }

    if v.len() == 1 {
        let n = v[0];
        let res = get_multiples(limit, n); // vector
        return get_sum(res);
    }
    for n in v {
        res.append(&mut get_multiples(limit, n));
    }
    get_sum(res)
}
