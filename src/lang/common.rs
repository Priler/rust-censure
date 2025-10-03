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

/// (order matters)
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

pub static PAT_SPACE: Lazy<Regex>       = Lazy::new(|| Regex::new(r"\s+").unwrap());

pub static PAT_PUNCT1: Lazy<Regex>      = Lazy::new(|| Regex::new(r#"["\-+;.,*?()]+"#).unwrap()); // zero-widths
pub static PAT_PUNCT2: Lazy<Regex>      = Lazy::new(|| Regex::new(r#"[!:_]+"#).unwrap());
pub static PAT_PUNCT3: Lazy<Regex>      = Lazy::new(|| Regex::new(r#"["\-+;.,*?()!:_]+"#).unwrap());

pub static PAT_HTML_TAG: Lazy<Regex>    = Lazy::new(|| Regex::new(r#"(<.*?>)|(&[\w]{2,6};)|(<![-]+)|([-]+>)"#).unwrap());
// pub static PAT_HTML_TAG_OR_SPACER: Lazy<Regex>    = Lazy::new(|| Regex::new(r#"(?P<tag><.*?>)|(?P<spacer>[\s]+)"#).unwrap());
// pub static PAT_HTML_CSS: Lazy<Regex>  = Lazy::new(|| Regex::new(r"[\w\s}{.#;:\-+]").unwrap());
pub static PAT_HTML_SPACE: Lazy<Regex>  = Lazy::new(|| Regex::new(r"(?i)&nbsp;").unwrap());
pub static PAT_PREP: Lazy<Regex>               = Lazy::new(|| Regex::new(r"(а[х]?)|(в)|([вмт]ы)|(д[ао])|(же)|(за)").unwrap());