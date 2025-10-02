pub mod common;
pub mod ru;
pub mod en;

use crate::structs::LangData;

pub trait LangProvider {
    fn data() -> &'static LangData;
}