use fancy_regex::Regex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use crate::censor::LangData;

/// Pulling from constants.py: FOUL_DATA, FOUL_CORE, EXCLUDES_DATA, EXCLUDES_CORE, BAD_*
/// You can paste the full maps verbatim; I show shortened examples and the mechanism.
/// :contentReference[oaicite:8]{index=8}
pub fn build_en_data() -> LangData {
    use fancy_regex::Regex;

    // Helper to compile a Vec<&str> -> Vec<Regex>
    let compile_vec = |v: Vec<&'static str>| -> Vec<Regex> {
        v.into_iter().map(|s| Regex::new(s).unwrap()).collect()
    };

    // Helper to compile a HashMap<char, Vec<&str>> -> HashMap<char, Vec<Regex>>
    let compile_bucket = |m: HashMap<char, Vec<&'static str>>| -> HashMap<char, Vec<Regex>> {
        m.into_iter().map(|(k, v)| (k, compile_vec(v))).collect()
    };

    // --- FOUL_CORE (dict of key->pattern OR just a flat dict) ---
    let foul_core_raw: HashMap<&'static str, &'static str> = HashMap::from([
        // examples (paste all from FOUL_CORE):
        ("бзд", r"бзд"),
        ("бля", r"(б[еи]?л[у]?я)|(бля)|(блиад)|(бл[еи]д[иу])|([кю]ляд)|([бп][и]?л[ая][дт]ь$)|(билат.?$)|(блдс)"),
        // ...
    ]);
    let foul_core = foul_core_raw
        .into_iter()
        .map(|(k, v)| (k.to_string(), Regex::new(v).unwrap()))
        .collect::<HashMap<String, Regex>>();

    // --- FOUL_DATA by first letter ---
    let foul_data_raw: HashMap<char, Vec<&'static str>> = HashMap::from([
        ('а', vec![r"^абанамат", r"^анахну", r"^архипиздрит", r"^аст[ао]еб"]),
        ('б', vec![r"^б[еи]л[еи]ат", r"^бабоеб", r"^басран", r"^баться", r"^бзд"]),
        // ... paste all entries
    ]);
    let foul_data = compile_bucket(foul_data_raw);

    // --- EXCLUDES_CORE ---
    let excludes_core_raw: HashMap<&'static str, &'static str> = HashMap::from([
        ("боле", r"боле"),
        ("гре#1", r"^.{0,3}греб$"),
        ("мандат", r"мандат(?!(в))"),
        // ... paste all entries
    ]);
    let excludes_core = excludes_core_raw
        .into_iter()
        .map(|(k, v)| (k.to_string(), Regex::new(v).unwrap()))
        .collect::<HashMap<String, Regex>>();

    // --- EXCLUDES_DATA by first letter ---
    let excludes_data_raw: HashMap<char, Vec<&'static str>> = HashMap::from([
        ('а', vec![r"автомоб", r"амеб", r"амудар", r"ансамбл"]),
        ('б', vec![r"(бал?)", r"(бу?)(бу?)+", r"бебел", r"бедр", r"белибер"]),
        // ... paste all entries
    ]);
    let excludes_data = compile_bucket(excludes_data_raw);

    // --- BAD phrases ---
    let bad_semi_phrases = compile_vec(vec![
        r"анепош[е]?л[и]?бы[вт]ы", r"еханыйбабай", r"идинахуй",
        r"имел[аи]?тебя(?!(ввид))", r"отс[ао]сатьу", r"отс[ао]сичлен",
        r"редк..педальност", r"сучийпотрох", r"тебяимел[аи]?(?!(ввид))", r"тык[ао]зел",
    ]);
    let bad_phrases: Vec<Regex> = vec![]; // BAD_PHRASES is empty in the snippet

    LangData {
        beep: super::super::common::BEEP.to_string(),
        beep_html: super::super::common::BEEP_HTML.to_string(),
        foul_data,
        foul_core,
        excludes_data,
        excludes_core,
        bad_semi_phrases,
        bad_phrases,
    }
}
