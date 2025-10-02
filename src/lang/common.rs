use fancy_regex::Regex;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// The censor 'beep' words
pub static BEEP: &str = "***";
pub static BEEP_HTML: &str = "<span class=\"censored\">***</span>";

/// Utility: apply TRANS_TAB
pub fn translate_similar_chars(s: &str, trans_table: &HashMap<char, char>) -> String {
    s.chars().map(|c| trans_table.get(&c).copied().unwrap_or(c)).collect()
}

/// patterns.PATTERNS_REPLACEMENTS from patterns.py (order matters) :contentReference[oaicite:5]{index=5}
pub static NORMALIZATION_PATTERNS: Lazy<Vec<(Regex, &'static str)>> = Lazy::new(|| {
    vec![
        (Regex::new(r"&[Ee][Uu][Mm][Ll];").unwrap(), "е"),
        (Regex::new(r"&[Uu][Uu][Mm][Ll];").unwrap(), "и"),
        (Regex::new(r"&[Aa][Uu][Mm][Ll];").unwrap(), "а"),
        (Regex::new(r"&[Oo][Uu][Mm][Ll];").unwrap(), "о"),
        (Regex::new(r"&[Yy][Uu][Mm][Ll];").unwrap(), "у"),

        (Regex::new(r"&#203;").unwrap(), "е"),

        (Regex::new(r"&[Cc][Ee][Nn][Tt];").unwrap(), "с"),
        (Regex::new(r"&#162;").unwrap(), "с"),

        (Regex::new(r"\|\\\|").unwrap(), "и"), // И
        (Regex::new(r"/\\").unwrap(), "л"),    // Л

        (Regex::new(r"><").unwrap(), "х"),
        (Regex::new(r"\)\(").unwrap(), "х"),
        (Regex::new(r"}{").unwrap(), "х"),

        (Regex::new(r">\|<").unwrap(), "ж"),
        (Regex::new(r"}\|{").unwrap(), "ж"),

        (Regex::new(r"`/").unwrap(), "y"),
        (Regex::new(r"\-/").unwrap(), "y"),
        (Regex::new(r"`\-/").unwrap(), "y"),

        (Regex::new(r"b\|").unwrap(), "ы"),
        (Regex::new(r"bI").unwrap(), "ы"),
        (Regex::new(r"bl").unwrap(), "ы"),

        (Regex::new(r"&#120;").unwrap(), "х"),
        (Regex::new(r"&#121;").unwrap(), "у"),
        (Regex::new(r"3[\.,]14[\d]*").unwrap(), "пи"),
        (Regex::new(r"2[\.,]72[\d]*").unwrap(), "е"),
    ]
});

/// Simple reusable patterns that your Python uses from lang.common.patterns (not all were in the snippet).
/// We fill reasonable equivalents here for splitting/cleaning.
pub static PAT_SPACE: Lazy<regex::Regex>       = Lazy::new(|| regex::Regex::new(r"\s+").unwrap());
pub static PAT_PUNCT1: Lazy<regex::Regex>      = Lazy::new(|| regex::Regex::new(r"[\u200b\p{Cf}]").unwrap()); // zero-widths
pub static PAT_PUNCT2: Lazy<regex::Regex>      = Lazy::new(|| regex::Regex::new(r"[^\p{L}\p{N}<>\s/]+").unwrap());
pub static PAT_PUNCT3: Lazy<regex::Regex>      = Lazy::new(|| regex::Regex::new(r"^[^\p{L}\p{N}]+|[^\p{L}\p{N}]+$").unwrap());
pub static PAT_HTML_TAG: Lazy<regex::Regex>    = Lazy::new(|| regex::Regex::new(r"(?is)<[^>]+>").unwrap());
pub static PAT_HTML_SPACE: Lazy<regex::Regex>  = Lazy::new(|| regex::Regex::new(r"(?s)^\s*$").unwrap());
pub static PAT_PREP: Lazy<Regex>               = Lazy::new(|| Regex::new(r"(а[х]?)|(в)|([вмт]ы)|(д[ао])|(же)|(за)").unwrap()); // :contentReference[oaicite:6]{index=6}