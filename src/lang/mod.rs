pub mod common;
pub mod ru;
pub mod en;

pub(crate) use crate::structs::LangData;

pub trait LangProvider {
    fn data() -> LangData;
}