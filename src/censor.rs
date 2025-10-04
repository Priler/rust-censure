use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc};
use parking_lot::{RwLock, RwLockWriteGuard};

use super::structs::*;

use crate::lang::common::{
    NORMALIZATION_PATTERNS, PAT_PUNCT3
};
use crate::lang::LangProvider;
use crate::util::{remove_duplicates, is_pi_or_e_word};
use fancy_regex;

impl<'a, L: LangProvider> Censor<'a, L> {
    pub fn new(lang: &'a L) -> Result<Self, CensorError> {
        Ok(Self {
            lang: &lang,
            data: lang.data(),
            re_cache: Lazy::new(|| Arc::new(RwLock::new(HashMap::with_capacity(1000))))
        })
    }

    fn is_match_cached(&self, pat: &str, text: &str) -> bool {
        // Check cache
        {
            let cache = self.re_cache.read();
            if let Some(r) = cache.get(pat) {
                return r.is_match(text).unwrap_or(false)
            }
        }

        // Compile and cache
        let r = fancy_regex::Regex::new(pat)
            .map_err(|e| CensorError::RegexCompilationFailed(e.to_string())).unwrap();
        let res = r.is_match(text).unwrap_or(false);
        {
            let mut cache = self.re_cache.write();
            cache.insert(pat.to_string(), r);
        }
        res
    }

    // fn cache_pattern(&self, pat: &str, r: fancy_regex::Regex, cache: &mut std::sync::RwLockWriteGuard<HashMap<String, fancy_regex::Regex>>) {
    //     // Check cache
    //     if cache.contains_key(pat) {
    //         return // already cached
    //     }
    //
    //     cache.insert(pat.to_string(), r);
    // }

    fn compile_and_cache_pattern(&self, pat: &str, cache: &mut RwLockWriteGuard<HashMap<String, fancy_regex::Regex>>) {
        let r = fancy_regex::Regex::new(pat)
            .map_err(|e| CensorError::RegexCompilationFailed(e.to_string())).unwrap();
        cache.insert(pat.to_string(), r);
    }

    pub fn precompile_all_patterns(&self) {
        self.precompile_foul_data();
        self.precompile_foul_core();
        self.precompile_bad_phrases();
        self.precompile_bad_semi_phrases();
        self.precompile_excludes_core();
        self.precompile_excludes_data();
    }

    pub fn precompile_foul_data(&self) {
        let mut cache = self.re_cache.write();

        for (_, pats) in self.data.foul_data {
            for &pat in pats {
                self.compile_and_cache_pattern(pat, &mut cache);
            }
        }
    }

    pub fn precompile_foul_core(&self) {
        let mut cache = self.re_cache.write();

        for (pat, _) in self.data.foul_core {
            self.compile_and_cache_pattern(pat, &mut cache);
        }
    }

    pub fn precompile_bad_phrases(&self) {
        let mut cache = self.re_cache.write();

        for &pat in self.data.bad_phrases {
            self.compile_and_cache_pattern(pat, &mut cache);
        }
    }

    pub fn precompile_bad_semi_phrases(&self) {
        let mut cache = self.re_cache.write();

        for &pat in self.data.bad_semi_phrases {
            self.compile_and_cache_pattern(pat, &mut cache);
        }
    }

    pub fn precompile_excludes_core(&self) {
        let mut cache = self.re_cache.write();

        for (pat, _) in self.data.excludes_core {
            self.compile_and_cache_pattern(pat, &mut cache);
        }
    }

    pub fn precompile_excludes_data(&self) {
        let mut cache = self.re_cache.write();

        for (_, pats) in self.data.excludes_data {
            for &pat in pats {
                self.compile_and_cache_pattern(pat, &mut cache);
            }
        }
    }

    fn replace_all_cached(&self, pat: &str, text: &'a str, repl: &str) -> Option<String> {
        // Quick negative guard: if it doesn't match, skip compiling/allocating a String for replace.
        if !self.is_match_cached(pat, text) {
            return None;
        }

        // read from cache
        let cache = self.re_cache.read();
        let compiled = cache.get(pat).unwrap();

        // replace
        let replaced = compiled.replace_all(text, repl).into_owned();
        if replaced == text { None } else { Some(replaced) }
    }

    fn split_line(&self, s: &str) -> Vec<String> {
        self.lang.split_line(s)
    }

    fn prepare_word(&self, mut w: String) -> String {
        if !is_pi_or_e_word(&w) {
            // trim punctuation edges
            w = PAT_PUNCT3.replace_all(&w, "").into_owned();
        }
        let mut w = w.to_lowercase();

        // apply normalization patterns in order
        for (pat, rep) in NORMALIZATION_PATTERNS.iter() {
            w = pat.replace_all(&w, *rep).into_owned();
        }

        // transliteration of similar chars
        w = crate::lang::common::translate_similar_chars(&w, self.data.trans_tab);

        // deduplicate (AAA -> AA)
        remove_duplicates(&w)
    }

    pub fn is_word_good(&self, raw: &str) -> bool {
        let w = self.prepare_word(raw.to_string());
        self.check_word_impl_fast(&w)
    }

    fn check_word_impl(&self, prepared: &String) -> WordInfo {
        let mut info = WordInfo::new(Box::from(prepared.as_str()));

        // Build a string from the first character
        let fl_str = String::from(info.word.chars().next().map(|c| c.to_string()).unwrap_or_default());

        // 1) Accuse stage: FOUL_DATA[first_letter]
        if let Some(pats) = self.data.foul_data.get(fl_str.as_str()) {
            for &pat in pats {
                if self.is_match_cached(pat, &info.word) {
                    info.is_good = false;
                    info.accuse.push(Box::from(pat)); // now stored as a string rule
                    break;
                }
            }
        }

        // 2) If still good → check FOUL_CORE
        if info.is_good {
            for (&_key, &pat) in self.data.foul_core.iter() {
                if self.is_match_cached(pat, prepared) {
                    info.is_good = false;
                    info.accuse.push(Box::from(pat));
                    break;
                }
            }
        }

        // 3) If still good → check BAD_SEMI_PHRASES
        if info.is_good {
            for &pat in self.data.bad_semi_phrases.iter() {
                if self.is_match_cached(pat, prepared) {
                    info.is_good = false;
                    info.accuse.push(Box::from(pat));
                    break;
                }
            }
        }

        // 4) Excuse stage: if already accused, check exceptions
        if !info.is_good {
            // EXCLUDES_CORE
            for (&_key, &pat) in self.data.excludes_core.iter() {
                if self.is_match_cached(pat, prepared) {
                    info.is_good = true;
                    info.excuse.push(Box::from(pat));
                    break;
                }
            }
            // EXCLUDES_DATA[first_letter]
            if !info.is_good {
                if let Some(pats) = self.data.excludes_data.get(fl_str.as_str()) {
                    for &pat in pats {
                        if self.is_match_cached(pat, prepared) {
                            info.is_good = true;
                            info.excuse.push(Box::from(pat));
                            break;
                        }
                    }
                }
            }
        }

        info
    }

    fn check_word_impl_fast(&self, prepared: &str) -> bool {
        // Fast path: only check if word is good, no detailed info
        let fl_str = String::from(prepared.chars().next().map(|c| c.to_string()).unwrap_or_default());

        // 1) Accuse stage: FOUL_DATA[first_letter]
        if let Some(pats) = self.data.foul_data.get(fl_str.as_str()) {
            for &pat in pats {
                if self.is_match_cached(pat, prepared) {
                    return false;
                }
            }
        }

        // 2) If still good → check FOUL_CORE
        for (&_key, &pat) in self.data.foul_core.iter() {
            if self.is_match_cached(pat, prepared) {
                return false;
            }
        }

        // 3) If still good → check BAD_SEMI_PHRASES
        for &pat in self.data.bad_semi_phrases.iter() {
            if self.is_match_cached(pat, prepared) {
                return false;
            }
        }

        // 4) Excuse stage: if already accused, check exceptions
        // EXCLUDES_CORE
        for (&_key, &pat) in self.data.excludes_core.iter() {
            if self.is_match_cached(pat, prepared) {
                return true;
            }
        }
        // EXCLUDES_DATA[first_letter]
        if let Some(pats) = self.data.excludes_data.get(fl_str.as_str()) {
            for &pat in pats {
                if self.is_match_cached(pat, prepared) {
                    return true;
                }
            }
        }

        false // bad word
    }

    /// returns replaced line plus counts
    pub fn clean_line(&self, line: &str) -> CleanLineResult {
        // Mutable working buffer that accumulates changes
        let mut out = line.to_string();

        // Counters and diagnostics
        let mut bad_words = 0usize;
        let mut bad_phrases = 0usize;
        let mut detected_words: Vec<Box<str>> = Vec::with_capacity(5);
        let mut detected_pats = Vec::with_capacity(5);

        // 1) Word-by-word replacement (first hit per surface word):
        //
        // - Split the *original* line into tokens according to language rules.
        // - For each token, normalize and check with accuse/excuse logic.
        // - If bad, replace the *first* occurrence of the exact surface token in `out`.
        //   This preserves original casing/punctuation and mirrors your Python behavior.
        for word in self.split_line(line) {
            let prepared = self.prepare_word(word.clone());
            let info = self.check_word_impl(&prepared);
            if !info.is_good {
                bad_words += 1;
                out = out.replacen(&word, self.data.beep, 1);
                detected_words.push(Box::from(word.as_str()));
                if let Some(p) = info.accuse.get(0) {
                    detected_pats.push(p.clone());
                }
            }
        }

        // 2) Phrase-level replacements:
        //
        // - BAD_SEMI_PHRASES are broad patterns that run over the whole string.
        // - We first check via `is_match_cached` to avoid unnecessary work,
        //   then call `replace_all_cached` which compiles via the same cache.
        for &pat in self.data.bad_semi_phrases.iter() {
            if let Some(new_out) = self.replace_all_cached(pat, &out, self.data.beep) {
                bad_phrases += 1;
                detected_pats.push(Box::from(pat));
                out = new_out;
            }
        }

        // If you also maintain BAD_PHRASES, process them the same way:
        for &pat in self.data.bad_phrases.iter() {
            if let Some(new_out) = self.replace_all_cached(pat, &out, self.data.beep) {
                bad_phrases += 1;
                detected_pats.push(Box::from(pat));
                out = new_out;
            }
        }

        CleanLineResult {
            line: out,
            bad_words_count: bad_words,
            bad_phrases_count: bad_phrases,
            detected_bad_words: detected_words,
            detected_patterns: detected_pats,
        }
    }

    /// Clean an HTML string while preserving tags and replacing bad words with `beep_html`.
    /// @TODO: Rewrite the implementation, so it'll work with any HTML tags (incl broken etc).
    /// Use a proper HTML parser like scraper or kuchiki
    pub fn clean_html_line(&self, line: &str) -> CleanHtmlResult {
        use crate::html::{tokenize_html, TokType, Token};

        let tokens = tokenize_html(line);

        let mut current_word = String::new();            // plain word (no tags)
        let mut current_tagged = String::new();          // word with tags as text
        let mut tagged_list: Vec<&Token> = Vec::new();   // token objects for pre/post reconstruction

        let mut out = String::new();
        let mut bad_count = 0usize;

        let beep_html = self.data.beep_html; // HTML replacement for a bad word

        // Compute "pre" (opening + self-closing tags) and "post" (closing tags)
        // from the tokens collected for the current word.
        fn get_remained_tokens(tagged: &[&Token]) -> (String, String) {
            let mut pre = String::new();
            let mut post = String::new();

            for t in tagged {
                match t.kind {
                    TokType::TagOpen | TokType::TagSelf => {
                        // opening/self tags should remain before the censored placeholder
                        pre.push_str(&t.value);
                    }
                    TokType::TagClose => {
                        // closing tags should remain after the censored placeholder
                        post.push_str(&t.value);
                    }
                    _ => {}
                }
            }
            (pre, post)
        }

        // Flush the currently accumulated word (and its tag list) into `out`.
        // If the word is bad, we output `pre + beep_html + post`. Otherwise, we output the original tagged text.
        // Optionally append a trailing literal (space/spacer) after flushing.
        let process_spacer = |cw: &mut String,
                                  ctw: &mut String,
                                  twl: &mut Vec<&Token>,
                                  r: &mut String,
                                  bwc: &mut usize,
                                  tok: Option<&Token>| {
            if !cw.is_empty() {
                // println!("{}", cw);
                if !self.is_word_good(cw) {
                    let (pre, post) = get_remained_tokens(twl);
                    *r += &pre;
                    *r += beep_html;
                    *r += &post;
                    *bwc += 1;
                } else {
                    // Good word: emit the original tagged fragment unchanged
                    *r += ctw;
                }
            }
            // Reset per-word buffers
            twl.clear();
            cw.clear();
            ctw.clear();

            // Append trailing boundary (space/spacer) if provided
            if let Some(t) = tok {
                *r += &t.value;
            }
        };

        // Iterate over tokens exactly like the Python version
        for tok in &tokens {
            match tok.kind {
                TokType::TagOpen | TokType::TagClose | TokType::TagSelf => {
                    // Tags are part of the current "tagged word"; they do NOT trigger a flush
                    tagged_list.push(tok);
                    current_tagged.push_str(&tok.value);
                }
                TokType::Word => {
                    // Word fragments are appended to both plain and tagged buffers
                    // println!("current_word: {}", current_word);
                    if !self.is_word_good(&current_word) {
                        process_spacer(
                            &mut current_word,
                            &mut current_tagged,
                            &mut tagged_list,
                            &mut out,
                            &mut bad_count,
                            Some(tok),
                        );
                    } else {
                        tagged_list.push(tok);
                        current_tagged.push_str(&tok.value);
                        current_word.push_str(&tok.value);
                    }
                }
                TokType::Space |  TokType::Spacer => {
                    // Boundary: process the current word and then append the space/spacer
                    process_spacer(
                        &mut current_word,
                        &mut current_tagged,
                        &mut tagged_list,
                        &mut out,
                        &mut bad_count,
                        Some(tok),
                    );
                }
            }
        }

        // Final flush if the line ended without a trailing space
        if !current_word.is_empty() || !current_tagged.is_empty() {
            process_spacer(
                &mut current_word,
                &mut current_tagged,
                &mut tagged_list,
                &mut out,
                &mut bad_count,
                None,
            );
        }

        CleanHtmlResult { line: out, bad_words_count: bad_count }
    }
}
