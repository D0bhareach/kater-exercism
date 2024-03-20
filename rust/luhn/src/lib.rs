/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let s = code.trim();
    if s.len() <= 1 {
        return false;
    }

    let chars = s.chars().rev();
    let mut nums = 0;
    let mut total = 0;
    for c in chars {
        if c == ' ' {
            continue;
        }
        let mut d: usize = match c {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            '9' => 9,
            '0' => 0,
            _ => return false,
        };

        if nums % 2 != 0 {
            d = d * 2;

            if d > 9 {
                total += d - 9;
            } else {
                total += d;
            }
        } else {
            total += d;
        }

        nums += 1;
    }
    total % 10 == 0
}
