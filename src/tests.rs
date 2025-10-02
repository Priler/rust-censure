#[cfg(test)]
mod tests {
    use crate::{Censor, CensorLang};
    use crate::util::compile_any;
    use pretty_assertions::assert_eq;
    use crate::lang::{LangData, LangProvider};

    #[test]
    fn ru_test() {
        let censor = Censor::new(CensorLang::Ru).unwrap();
        let line = "сykА блядб";
        let res = censor.clean_line(line);

        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);
    }

    #[test]
    fn en_test() {
        let censor = Censor::new(CensorLang::En).unwrap();
        let line = "dumb ass";
        let res = censor.clean_line(line);

        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);
    }

    #[test]
    fn compile_all_ru_patterns() {
        compile_all_patterns(crate::lang::ru::Ru::data());
    }

    #[test]
    fn compile_all_en_patterns() {
        compile_all_patterns(crate::lang::en::En::data());
    }

    fn compile_all_patterns(d: &LangData) {
        for (k, v) in d.foul_core.iter() {
            compile_any(v).expect(&format!("FOUL_CORE key {} failed", k));
        }
        for (k, list) in d.foul_data.iter() {
            for pat in list {
                compile_any(pat).expect(&format!("FOUL_DATA[{}] pattern {:?} failed", k, pat));
            }
        }
        for (k, v) in d.excludes_core.iter() {
            compile_any(v).expect(&format!("EXCLUDES_CORE key {} failed", k));
        }
        for (k, list) in d.excludes_data.iter() {
            for pat in list {
                compile_any(pat).expect(&format!("EXCLUDES_DATA[{}] pattern {:?} failed", k, pat));
            }
        }
        for pat in d.bad_semi_phrases.iter() {
            compile_any(pat).expect(&format!("BAD_SEMI_PHRASES pattern {:?} failed", pat));
        }
        for pat in d.bad_phrases.iter() {
            compile_any(pat).expect(&format!("BAD_PHRASES pattern {:?} failed", pat));
        }
    }
}