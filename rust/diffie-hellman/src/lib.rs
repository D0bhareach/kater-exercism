// Implementation  of this exercise is half working. Must come back to it at the end,
// read links, but topic is hard, explanation Exercism site is not clear, even this
// implementation I have done it was BDD..
pub fn private_key(p: u64) -> u64 {
    // will work on assumption that p is always prime number
    let mut e = p - 1; // because of Euler's totient fn
    let mut totients: Vec<f64> = vec![]; // must remove dups from it
    let mut pows: Vec<f64> = vec![];
    let mut div = 2;
    while div < p {
        if e % div == 0 {
            totients.push(div as f64);
            e = e / div;
            pows.push(e as f64);
            continue;
        }
        div += 1;
    }
    // now when numbers are ready start to check modulus
    let mut res = 0; // because don't know how to break out of for with break
    totients.dedup();
    for i in totients.iter() {
        for y in pows.iter() {
            // https://stackoverflow.com/questions/70754273/rust-attempt-to-multiply-with-overflow-when-using-pow-function
            let x: u64 = (i.powf(*y) % p as f64) as u64;
            if 1 < x && x < p {
                res = x;
                break;
            }
        }
    }
    res
}

// I don't know how efficient, but say for sake of this exercise I will set my
// reducing ponit to 1_000_000
// p = modulus, g=base, a=exponent
// This function is working and passing reducto test, but tests with big numbers are not passing
// get out of memory error and tests are killed. (signal: 9, SIGKILL: kill)
//
// pub fn reducto(p: u64, g: u64, a: u64) -> u64 {
//
//     let mut exp: f64 = a as f64;
//     let mut c = 1f64;
//     let base = g as f64;
//     let pp = p as f64;
//     let mut res: f64 = 0.0;
//
//     while exp > 0.0 {
//         res = dbg!((c * base) % pp);
//         c = res;
//         exp -= 1.0;
//     }
//     res as u64
// }
//
// Not working https://en.wikipedia.org/wiki/Modular_exponentiation#Left-to-right_binary_method
// right to left binary
// pub fn reducto(p: u64, g: u64, a: u64) -> u64 {
//     let mut r = 1;
//     let mut base = dbg!(g % p);
//     let mut e = a;
//     while e > 0 {
//         if e % 2 == 1 {
//             r = dbg!((r * base) % p);
//         } else {
//             base = dbg!((base * base) % p);
//             e = e >> 1;
//         }
//     }
//     r
// }

/// a = private key
/// g = base
/// p = modulus
pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    // if g > 1000000 || a > 1000000 {
    //     return reducto(p, g, a);
    // }

    let pf = p as f64;
    let gf = g as f64;
    let af = a as f64;
    (gf.powf(af) % pf) as u64
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    // if b_pub > 1000000 || a > 1000000 {
    //     return reducto(p, b_pub, a);
    // }

    let pf = p as f64;
    let gf = b_pub as f64;
    let af = a as f64;
    (gf.powf(af) % pf) as u64
}
// https://en.wikipedia.org/wiki/Diffie%E2%80%93Hellman_key_exchange
// https://math.stackexchange.com/questions/124408/finding-a-primitive-root-of-a-prime-number
// https://math.berkeley.edu/~kpmann/encryption.pdf
