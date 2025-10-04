use std::collections::HashMap;
use std::sync::{Arc};
use parking_lot::RwLock;
use once_cell::sync::Lazy;
use thiserror::Error;
use crate::lang::LangProvider;

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

impl LangData {
    pub fn get_beep(&self) -> &str {
        self.beep
    }

    pub fn set_beep(&mut self, val: &'static str) -> &str {
        self.beep = val;
        self.beep
    }

    pub fn get_beep_html(&self) -> &str {
        self.beep_html
    }

    pub fn set_beep_html(&mut self, val: &'static str) -> &str {
        self.beep_html = val;
        self.beep_html
    }
}

#[derive(Clone, Copy, Debug)]
pub enum CensorLang { Ru, En }

pub struct Censor {
    pub lang: Arc<dyn LangProvider + Send + Sync + 'static>,
    pub data: LangData,
    pub re_cache: Lazy<Arc<RwLock<HashMap<String, fancy_regex::Regex>>>>
}

#[derive(Debug, Error)]
pub enum CensorError {
    #[error("unsupported language: {0}")]
    UnsupportedLang(String),

    #[error("regex compilation failed: {0}")]
    RegexCompilationFailed(String)
}

#[derive(Debug)]
pub struct WordInfo {
    pub is_good: bool,
    pub word: Box<str>,
    pub accuse: Vec<Box<str>>,
    pub excuse: Vec<Box<str>>,
}
impl WordInfo {
    pub fn new(word: Box<str>) -> Self {
        Self { is_good: true, word, accuse: vec![], excuse: vec![] }
    }
}

#[derive(Debug)]
pub struct CleanLineResult {
    pub line: String,
    pub bad_words_count: usize,
    pub bad_phrases_count: usize,
    pub detected_bad_words: Vec<Box<str>>,
    pub detected_patterns: Vec<Box<str>>,
}

#[derive(Debug)]
pub struct CleanHtmlResult {
    pub line: String,
    pub bad_words_count: usize,
}