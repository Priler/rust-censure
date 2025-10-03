pub fn remove_duplicates(word: &str) -> String {
    // Keep at most two consecutive identical chars
    let mut out = String::with_capacity(word.len());
    let mut prev = '\0';
    let mut count = 0usize;
    for ch in word.chars() {
        if ch == prev {
            count += 1;
            if count < 3 {
                out.push(ch);
            }
        } else {
            prev = ch;
            count = 1;
            out.push(ch);
        }
    }
    out
}

pub fn is_pi_or_e_word(s: &str) -> bool {
    s.contains("2.72") || s.contains("3.14")
}
