pub mod common;
pub mod ru;
pub mod en;

pub(crate) use crate::structs::LangData;

pub trait LangProvider {
    fn data(&self) -> LangData;
    fn split_line(&self, line: &str) -> Vec<String>;
}