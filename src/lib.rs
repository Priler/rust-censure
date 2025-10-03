mod structs;
mod html;
mod lang;
mod censor;
mod util;

#[cfg(debug_assertions)]
mod tests;

pub use structs::{Censor, CensorLang, CleanLineResult, CleanHtmlResult, CensorError};
