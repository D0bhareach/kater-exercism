use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // 7! = 5040.
    fn to_lower<'a>(word: &'a str) -> Vec<char> {
        let v: Vec<char> = word
            .chars()
            .map(|c| {
                if c.is_uppercase() {
                    c.to_lowercase()
                        .last()
                        .expect("Inner func, converting char to lowercase.")
                } else {
                    c
                }
            })
            .collect();
        v
    }
    // prepare once.
    // unordered vector of word param. Need it for ignore case equality
    let vuo = to_lower(word);
    // must clone to get different vector. It will be sorted to test for
    // permutations.
    let mut v1 = vuo.clone();
    v1.sort_unstable();
    let mut res: HashSet<&'a str> = HashSet::new();

    for w in possible_anagrams.iter() {
        let mut v2 = to_lower(w);
        if vuo != v2 {
            v2.sort_unstable();
            if v1 == v2 {
                // don't care for returned bool
                res.insert(w);
            }
        }
    }
    res
}
