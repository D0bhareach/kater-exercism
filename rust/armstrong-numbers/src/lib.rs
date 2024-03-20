pub fn is_armstrong_number(num: u32) -> bool {
    // handle special case when num is single digit
    if num < 10 {
        return true;
    }

    // get ranks quantity
    let mut n = num;
    let x = ((num as f64).log(10u32.into()) + 1.0) as u32;
    let mut addendums: Vec<u32> = vec![];

    // loop num
    for i in (1..x).rev() {
        let divider = 10_u32.pow(i); // for getting ranks
        let z = (n / divider) as u32; // get front digit
        let y = ((n / divider) as u32).pow(x); // calculate addendum
        addendums.push(y);

        // reduce given num
        if divider >= 10 {
            n = n - z * divider;
            // handle last digit
            if n < 10 {
                addendums.push(n.pow(x));
                break;
            }
        }
    }
    let t = addendums.iter().sum();
    num == t
}

/*
 * */
