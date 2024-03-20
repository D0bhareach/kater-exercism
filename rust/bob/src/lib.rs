pub fn reply(message: &str) -> &str {
    // trim white spaces and special chars
    let trms: &[_] = &['\r', ' ', '\t'];
    let m = message.trim();
    let m = m.trim_end_matches(trms);

    // Need it later to check punctuation. And later it will come handy in match.
    let mut end = String::new();
    let mut m = m.to_string(); // Use strings methods later and reassign
    if m.is_empty() {
        return "Fine. Be that way!"; // Handle empty string.
    } else {
        if let Some(c) = m.pop() {
            if c.is_ascii_punctuation() {
                end.push(c);
            }
            m = m
                .chars()
                .filter(|a| a.is_ascii_alphabetic())
                .collect::<String>(); // remove all but alphabetic
        }
    }

    // m contains only letters now. Check if they all uppercase
    let mut is_all_upper = false;
    for c in m.chars() {
        is_all_upper = c.is_ascii_uppercase();
        if !is_all_upper {
            break;
        }
    }

    // handle all cases for punctuation and uppercases
    match end.pop() {
        Some(c) if c == '.' => "Whatever.",
        Some(c) if c == '!' => {
            if is_all_upper {
                "Whoa, chill out!"
            } else {
                "Whatever."
            }
        }
        Some(c) if c == '?' => {
            if is_all_upper {
                "Calm down, I know what I'm doing!"
            } else {
                "Sure."
            }
        }
        _ => {
            if is_all_upper {
                "Whoa, chill out!"
            } else {
                "Whatever."
            }
        }
    }
}
