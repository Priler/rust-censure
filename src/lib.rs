mod structs;
mod html;
pub mod lang;
mod censor;
mod util;

#[cfg(debug_assertions)]
mod tests;

pub use structs::{Censor, CleanLineResult, CleanHtmlResult, CensorError, CensorLang};
