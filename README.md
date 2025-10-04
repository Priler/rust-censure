[rust-censure](https://crates.io/crates/rust-censure) is a Rust port of the Python library [py-censure](https://github.com/masteroncluster/py-censure).

Currently, it supports two languages: Russian and English.  
But you can add new languages by implementing the `LangProvider` trait
_(check out the `lang/ru` and `lang/en` modules for examples)_  

In terms of optimization, the crate does two things:
1. Uses [fancy-regex](https://docs.rs/fancy-regex/latest/fancy_regex/) for complex patterns *(backrefs, lookarounds etc)*, and it delegates simple patterns to [regex](https://docs.rs/regex/latest/regex/)  
2. Compiles and caches regexes on demand

It’s still basically a beta version, but it should already work reliably.  
Note that this port does not follow the same rules as the original library.  
Some tweaks were implemented.

### Basic usage example:
```rust
use rust_censure::{Censor, CensorLang};
use rust_censure::lang::{en::Ru}; // say that's your custom language

let censor = Censor::new(CensorLang::En).unwrap(); // make Censor from default language providers
let ru_censor = Censor::from(Ru::new()).unwrap(); // or make it from your custom language provider

let line = "dumb ass";
let res = censor.clean_line(line);

assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
assert_eq!(res.bad_words_count, 2);
assert_eq!(res.bad_phrases_count, 0);
assert!(res.detected_bad_words.iter().map(|b| b.as_ref()).eq(["dumb", "ass"]));
```

### Performance benchmark
A simple straightforward benchmark is included in the `benches` folder.  
Compared to the original Python library, it's ~9x faster.

But take it with a grain of salt, as the benchmark is not a real-world use case.  
It really depends on the size of the input text and the complexity of the patterns.

Most of the patterns _(~95% currently)_ are handled by [regex](https://docs.rs/regex/latest/regex/), thus the overhead should be minimal.  
The worst case regex searches should have worst-case complexity of `O(m * n)`.  
(*where `m` is proportional to the size of the regex and `n` is proportional to the size of the string being searched*)  

Although for a bunch of complex patterns, the overhead can be noticeable.  
See [fancy-regex](https://docs.rs/fancy-regex/latest/fancy_regex/) for the more details.


### ToDo
- [ ] Support of [aho-corasick](https://docs.rs/aho-corasick/latest/aho_corasick/) for faster simple replaces
- [ ] Better HTML handling
- [ ] Real-world data tests
