use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut chars = input.graphemes(true).collect::<Vec<&str>>();
    let len = chars.len();

    for i in 0..len / 2 {
        chars.swap(i, len - 1 - i)
    }
    chars.concat()
}
