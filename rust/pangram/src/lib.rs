/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let arr = sentence.chars().fold([0_u8; 26], |mut acc, c|{
        if c.is_ascii_alphabetic() {
            let ch = c.to_ascii_lowercase();
            let idx: usize = ((ch as u8) -97) as usize;
            let count = acc[idx];
            if count == 0 {acc[idx] = 1}        
        }
        acc
    });
    arr.into_iter().sum::<u8>() == 26_u8
}
