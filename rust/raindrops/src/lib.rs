pub fn raindrops(n: u32) -> String {
    let mut res = String::new();
    let mut b = true;

    if n % 3 == 0 {
        res.push_str("Pling");
        b = if b { !b } else { b };
    }
    if n % 5 == 0 {
        res.push_str("Plang");
        b = if b { !b } else { b };
    }
    if n % 7 == 0 {
        res.push_str("Plong");
        b = if b { !b } else { b };
    }
    if b {
        res.push_str(&format!("{}", n));
    }
    res
}
