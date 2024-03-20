pub fn abbreviate(s: &str) -> String {
    let res = s.split_terminator([' ', '-', '_']).fold(
        (true, String::with_capacity(s.len())),
        |mut state, word| {
            if state.0 {
                match word {
                    mut w if word.ends_with(':') => {
                        state.0 = false; // match is found.
                        w = w.trim_end_matches(':');
                        state.1 = w.to_uppercase();
                    }
                    // don't like this run on all iterations and strings.
                    w if word.chars().all(|c| c.is_ascii_uppercase()) => {
                        if let Some(x) = w.chars().rev().last() {
                            state.1.push(x);
                        }
                    }
                    _ => {
                        word.char_indices()
                            .filter(|(idx, c)| idx == &0 || c.is_ascii_uppercase())
                            .for_each(|(_, c)| state.1.push(c.to_ascii_uppercase()));
                    }
                }
            }
            state
        },
    );

    res.1.trim_end().to_owned()
}
