// use unicode_segmentation::UnicodeSegmentation;
// pub fn reverse(input: &str) -> String {
//     UnicodeSegmentation::graphemes(input, true)
//         .rev()
//         .collect::<String>()
// }
//
pub fn reverse(input: &str) -> String {
    input.chars().rev().collect::<String>()
}
