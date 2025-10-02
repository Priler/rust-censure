[rust-censure](https://crates.io/crates/rust-censure) is a Rust port of the Python library [py-censure](https://github.com/masteroncluster/py-censure).

Currently, it supports two languages: Russian and English.
But you can add new languages by implementing the `LangProvider` trait
(check out the `ru` and `en` modules for examples)  

In terms of optimization, the crate does two things:
1. Uses [regex](https://docs.rs/regex/latest/regex/) for simple patterns and [fancy-regex](https://docs.rs/fancy-regex/latest/fancy_regex/) as a fallback for more complex ones (backrefs, lookarounds etc)  
2. Compiles and caches regexes on demand

It’s still basically a beta version, but it should already work reliably.  
Note that this port does not follow the same rules as the original library.  
Some tweaks were implemented.

### Basic usage example:
```rust
use rust_censure::{Censor, CensorLang};

let en_censor = Censor::new(CensorLang::En).unwrap();
let line = "dumb ass";
let res = en_censor.clean_line(line);

assert_eq!(res.line, format!("{0} {0}", en_censor.data.beep));
assert_eq!(res.bad_words_count, 2);