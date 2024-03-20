use std::collections::HashMap;

// this unswer is totally inspired by comunity solution. My was correct, but took
// too many lines.
pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    let mut map = nucleotide_counts(dna)?;
    map.remove(&nucleotide).ok_or(nucleotide)
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    // allocate at once
    let mut res = HashMap::with_capacity(4);
    ['A', 'C', 'G', 'T']
        .iter()
        .fold(None, |_, c| res.insert(*c, 0usize));
    for c in dna.chars() {
        // think when in chain I need some result it's either Option or Result!
        res.get_mut(&c).map(|i| *i += 1).ok_or(c)?;
    }
    Ok(res)
}
