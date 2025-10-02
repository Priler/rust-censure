use std::collections::HashMap;
use std::sync::RwLock;
use once_cell::sync::Lazy;
use thiserror::Error;
use crate::censor::AnyRegex;

#[derive(Debug)]
pub struct LangData {
    pub beep: &'static str,
    pub beep_html: &'static str,

    pub foul_data: &'static HashMap<&'static str, Vec<&'static str>>,
    pub foul_core: &'static HashMap<&'static str, &'static str>,
    pub excludes_data: &'static HashMap<&'static str, Vec<&'static str>>,
    pub excludes_core: &'static HashMap<&'static str, &'static str>,
    pub bad_semi_phrases: &'static Vec<&'static str>,
    pub bad_phrases: &'static Vec<&'static str>,
    pub trans_tab: &'static HashMap<char, char>,
}

#[derive(Clone, Copy, Debug)]
pub enum CensorLang { Ru, En }

pub struct Censor {
    pub lang: CensorLang,
    pub data: &'static LangData,
    pub re_cache: Lazy<RwLock<HashMap<String, AnyRegex>>>
}

#[derive(Debug, Error)]
pub enum CensorError {
    #[error("unsupported language: {0}")]
    UnsupportedLang(String),
}

#[derive(Debug)]
pub struct WordInfo {
    pub is_good: bool,
    pub word: String,
    pub accuse: Vec<String>,
    pub excuse: Vec<String>,
}
impl WordInfo {
    pub fn new(word: String) -> Self {
        Self { is_good: true, word, accuse: vec![], excuse: vec![] }
    }
}

#[derive(Debug)]
pub struct CleanLineResult {
    pub line: String,
    pub bad_words_count: usize,
    pub bad_phrases_count: usize,
    pub detected_bad_words: Vec<String>,
    pub detected_patterns: Vec<String>,
}

#[derive(Debug)]
pub struct CleanHtmlResult {
    pub line: String,
    pub bad_words_count: usize,
}