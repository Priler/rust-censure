pub mod common;
pub mod ru;
pub mod en;

use std::sync::Arc;
pub(crate) use crate::structs::LangData;

pub trait LangProvider {
    fn data(&self) -> LangData;
    fn split_line(&self, line: &str) -> Vec<String>;
}