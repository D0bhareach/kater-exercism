/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    // with no particular reason I would like to have string processing without
    // unnecessary allocation and initial str preprocessing with later call for len

    let (flag, idx, total) = isbn
        .chars()
        .filter(|c| !c.eq(&'-'))
        .rev()
        .scan((false, 10u32, 0u32), move |state, ch| {
            if state.2.eq(&0) {
                state.0 = true;
            }
            if state.0 && state.1.gt(&0) {
                let d = match ch.to_digit(10) {
                    Some(d) => d,
                    None => {
                        if state.1.eq(&10) && (ch.eq(&'X') || ch.eq(&'x')) {
                            10
                        } else {
                            state.0 = false;
                            12
                        }
                    }
                };
                state.2 += d * state.1;
                state.1 -= 1;
            } else {
                state.0 = false;
            }
            Some(*state)
        })
        .last()
        .unwrap_or((false, 0, 1));

    flag && idx.eq(&0) && total.rem_euclid(11) == 0
}
