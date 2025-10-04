use crate::lang::common::{PAT_PUNCT1, PAT_PUNCT2, PAT_SPACE};
use super::LangProvider;
use crate::structs::LangData;

mod data;
pub struct En;

impl LangProvider for En {
    fn data(&self) -> LangData {
        LangData {
            beep: super::common::BEEP,
            beep_html: super::common::BEEP_HTML,

            foul_data: &crate::lang::en::data::FOUL_DATA,
            foul_core: &crate::lang::en::data::FOUL_CORE,
            excludes_data: &crate::lang::en::data::EXCLUDES_DATA,
            excludes_core: &crate::lang::en::data::EXCLUDES_CORE,
            bad_semi_phrases: &crate::lang::en::data::BAD_SEMI_PHRASES,
            bad_phrases: &crate::lang::en::data::BAD_PHRASES,
            trans_tab: &crate::lang::en::data::TRANS_TAB
        }
    }

    fn split_line(&self, line: &str) -> Vec<String> {
        let step1 = PAT_PUNCT1.replace_all(line, "");
        let step2 = PAT_PUNCT2.replace_all(&step1, " ");
        let mut buf = String::with_capacity(16);
        let mut out = Vec::with_capacity(8);

        for w in PAT_SPACE.split(&step2) {
            let w = w.unwrap();

            if w.is_empty() { continue; }
            if w.chars().count() < 3 {
                buf.push_str(w);
            } else {
                if !buf.is_empty() {
                    out.push(std::mem::take(&mut buf));
                }
                out.push(w.to_string());
            }
        }
        if !buf.is_empty() { out.push(buf); }
        out
    }
}