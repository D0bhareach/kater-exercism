fn check_next(v: &[u8], idx: usize) -> bool {
    let m = v[idx as usize];
    let n = v[(idx + 1) as usize];
    return (m == 40_u8 && n == 41_u8)
        || (m == 91_u8 && n == 93_u8)
        || (m == 123_u8 && n == 125_u8);
}

pub fn brackets_are_balanced(string: &str) -> bool {
    if string.len() == 0 {
        return true;
    }
    let brackets = vec![b'(', b')', b'[', b']', b'{', b'}'];
    let v: Vec<u8> = string.bytes().filter(|c| brackets.contains(c)).collect();
    let vl = v.len();
    if vl % 2 == 1 {
        return false; // extra bracket without pair
    }
    if vl == 2 {
        // no point to walk long walk.
        return check_next(&v, 0_usize);
    }

    let mut index = 0;
    let loop_exit = vl / 2;
    let mut res = true;
    let mut back_step = 1;
    let mut success = 0;

    println!("v: {:?}", v);
    while success < loop_exit {
        println!("{back_step}");
        res = check_next(&v, index); // check next bracket could be pair
        if res {
            index = index + 2;
            success += 1;
            continue;
        }

        // must be bracket on the end of sequence,than
        let m = v[index];
        let n = v[vl - back_step];
        res = (m == 40_u8 && n == 41_u8)
            || (m == 91_u8 && n == 93_u8)
            || (m == 123_u8 && n == 125_u8);

        if res {
            index += 1;
            back_step += 1;
            success += 1;
            continue;
        } else {
            return false; // By this time we didn't find math so its not balanced.
        }
    }
    res
}
