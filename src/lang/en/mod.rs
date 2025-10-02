use once_cell::sync::Lazy;
use super::LangProvider;
use crate::structs::LangData;

pub mod data;
pub struct En;

impl LangProvider for En {
    fn data() -> &'static LangData {
        static DATA: Lazy<LangData> = Lazy::new(|| LangData {
            beep: super::common::BEEP,
            beep_html: super::common::BEEP_HTML,

            foul_data: &crate::lang::en::data::FOUL_DATA,
            foul_core: &crate::lang::en::data::FOUL_CORE,
            excludes_data: &crate::lang::en::data::EXCLUDES_DATA,
            excludes_core: &crate::lang::en::data::EXCLUDES_CORE,
            bad_semi_phrases: &crate::lang::en::data::BAD_SEMI_PHRASES,
            bad_phrases: &crate::lang::en::data::BAD_PHRASES,
            trans_tab: &crate::lang::en::data::TRANS_TAB
        });

        &DATA
    }
}