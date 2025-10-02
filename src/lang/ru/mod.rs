use once_cell::sync::Lazy;
use super::LangProvider;
use crate::structs::LangData;

pub mod data;
pub struct Ru;

impl LangProvider for Ru {
    fn data() -> &'static LangData {
        static DATA: Lazy<LangData> = Lazy::new(|| LangData {
            beep: super::common::BEEP,
            beep_html: super::common::BEEP_HTML,

            foul_data: &data::FOUL_DATA,
            foul_core: &data::FOUL_CORE,
            excludes_data: &data::EXCLUDES_DATA,
            excludes_core: &data::EXCLUDES_CORE,
            bad_semi_phrases: &data::BAD_SEMI_PHRASES,
            bad_phrases: &data::BAD_PHRASES,
            trans_tab: &data::TRANS_TAB
        });

        &DATA
    }
}