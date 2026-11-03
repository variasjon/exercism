use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    let mut reversed = String::new();
    let chars = input.graphemes(true).collect::<Vec<&str>>();

    for i in (0..chars.len()).rev() {
        reversed.push_str(chars[i]);
    }
    reversed
}
