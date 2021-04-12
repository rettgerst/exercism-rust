use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    input
        .graphemes(true)
        .collect::<Vec<&str>>()
        .into_iter()
        .rev()
        .collect()
}
