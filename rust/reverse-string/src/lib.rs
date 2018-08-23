extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation as useg;
use std::collections::VecDeque;

pub fn reverse(input: &str) -> String {
    useg::graphemes(input, true)
        .fold(VecDeque::new(), |mut rev, g| {
            rev.push_front(g);
            rev
        })
        .iter()
        .fold("".to_string(), |mut s, g| {
            s.push_str(g);
            s
        })
}
