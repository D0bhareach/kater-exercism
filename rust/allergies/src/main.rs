fn main() {
    // let b = 1_u8 << 2;
    let val = 509;
    let f: Vec<u8> = format!("{val:b}").into();
    let mut n: Vec<u8> = f.into_iter().rev().collect();
    n.resize(8, 48);
    println!("{n:?}");
    /*
    let v: Vec<String> = f
        .char_indices()
        .map(|(i, c)| match (i, c) {
            (0, x) if x.eq(&'1') => "Strawberries".to_owned(),
            (1, x) if x.eq(&'1') => "Shell".to_owned(),
            (2, x) if x.eq(&'1') => "Eggs".to_owned(),
            (_, '0') => String::new(),
            (_, _) => String::new(),
        })
        .filter(|s| !s.is_empty())
        .collect();

    println!("{v:?}");
    */
}
