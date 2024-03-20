pub fn check(candidate: &str) -> bool {
    if candidate.len() == 0 {
        return true;
    }

    let i = candidate
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .map(|c| c.to_ascii_lowercase())
        .map(|c| c as u8);

    let mut b: u32 = 0b0;
    for c in i {
        let shift = c - 96;
        let x: u32 = 1 << shift;
        if (b & x) > 0 {
            return false;
        } else {
            b |= x;
        }
    }
    true
}
