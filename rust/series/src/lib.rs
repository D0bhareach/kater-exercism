pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut res: Vec<String> = Vec::new();
    let length = digits.len();
    if len == 0 {
        res = vec!["".to_string(); length + 1]; // one for each letter and 1 for whole str
    } else if len == length {
        res.push(digits.to_string());
    } else if length > len {
        let mut s = String::from(digits);
        // after all special cases loop over string, take slices
        // and reduce string by one from the left until it fits in len.
        while s.len() >= len {
            let sl = s.split_at(len as usize);
            res.push(sl.0.to_string());
            s.replace_range(..1_usize, "");
        }
    }

    res
}
