use crate::censor::AnyRegex;

use regex as regex_simple;
use fancy_regex as regex_fancy;

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

pub fn compile_any(pattern: &str) -> Result<AnyRegex, String> {
    // Try the fast regex crate first
    if let Ok(r) = regex_simple::Regex::new(pattern) {
        Ok(AnyRegex::Simple(r))
    } else {
        // Fall back to fancy-regex if the pattern needs lookarounds/backrefs
        if let Ok(r) = regex_fancy::Regex::new(pattern) {
            Ok(AnyRegex::Fancy(r))
        } else {
            Err(format!("Regex parse failed at pattern {:?}", pattern ))
        }
    }
}