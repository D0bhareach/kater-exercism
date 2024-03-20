pub fn verse(n: u32) -> String {
    let m = n.checked_sub(1).unwrap_or(0);
    // need add prefix line

    const BOTTLES: &str = "# bottles of beer on the wall, # bottles of beer.
Take one down and pass it around, # bottles of beer on the wall.\n";

    match n {
        0 => "No more bottles of beer on the wall, no more bottles of beer.
Go to the store and buy some more, 99 bottles of beer on the wall.\n"
            .to_string(),
        1 => {
            let mut tmp = BOTTLES
                .char_indices()
                .map(|(i, c)| match i {
                    0 | 31 => '1',
                    8 | 39 => char::default(),
                    84 => '0',
                    92 => 's',
                    _ => c,
                })
                .filter(|c| *c != char::default())
                .collect::<String>();
            tmp = tmp.replace("0", "no more");
            tmp.replacen("one", "it", 1)
        }
        2 => BOTTLES
            .char_indices()
            .map(|(i, c)| match i {
                0 | 31 => '2',
                8 | 39 => 's',
                84 => '1',
                92 => char::default(),
                _ => c,
            })
            .filter(|c| *c != char::default())
            .collect::<String>(),
        _ => BOTTLES
            .char_indices()
            .map(|(i, c)| match i {
                0 | 31 => char::from_digit(n as u32, 10).unwrap(),
                8 | 39 => 's',
                84 => char::from_digit(m as u32, 10).unwrap(),
                92 => 's',
                _ => c,
            })
            .filter(|c| *c != char::default())
            .collect::<String>(),
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut res = String::with_capacity(130);
    for n in (end..=start).rev() {
        res.push_str(&verse(n));
        if n != end {
            res.push('\n');
        }
    }
    res
}
