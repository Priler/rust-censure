#[cfg(test)]
mod tests {
    use crate::{Censor, CensorLang};
    use crate::util::compile_any;
    use pretty_assertions::assert_eq;

    #[test]
    fn ru_test() {
        let censor = Censor::new(CensorLang::Ru).unwrap();
        let line = "ебанамат бляд";
        let res = censor.clean_line(line);

        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);
    }

    #[test]
    fn en_test() {
        let censor = Censor::new(CensorLang::En).unwrap();
        let line = "bitch fuck";
        let res = censor.clean_line(line);

        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);
    }

    #[test]
    fn compile_all_ru_patterns() {
        use crate::lang::ru::data;
        for (k, v) in data::FOUL_CORE.iter() {
            compile_any(v).expect(&format!("FOUL_CORE key {} failed", k));
        }
        for (k, list) in data::FOUL_DATA.iter() {
            for pat in list {
                compile_any(pat).expect(&format!("FOUL_DATA[{}] pattern {:?} failed", k, pat));
            }
        }
        for (k, v) in data::EXCLUDES_CORE.iter() {
            compile_any(v).expect(&format!("EXCLUDES_CORE key {} failed", k));
        }
        for (k, list) in data::EXCLUDES_DATA.iter() {
            for pat in list {
                compile_any(pat).expect(&format!("EXCLUDES_DATA[{}] pattern {:?} failed", k, pat));
            }
        }
        for pat in data::BAD_SEMI_PHRASES.iter() {
            compile_any(pat).expect(&format!("BAD_SEMI_PHRASES pattern {:?} failed", pat));
        }
        for pat in data::BAD_PHRASES.iter() {
            compile_any(pat).expect(&format!("BAD_PHRASES pattern {:?} failed", pat));
        }
    }

    #[test]
    fn compile_all_en_patterns() {
        use crate::lang::en::data;
        for (k, v) in data::FOUL_CORE.iter() {
            compile_any(v).expect(&format!("FOUL_CORE key {} failed", k));
        }
        for (k, list) in data::FOUL_DATA.iter() {
            for pat in list {
                compile_any(pat).expect(&format!("FOUL_DATA[{}] pattern {:?} failed", k, pat));
            }
        }
        for (k, v) in data::EXCLUDES_CORE.iter() {
            compile_any(v).expect(&format!("EXCLUDES_CORE key {} failed", k));
        }
        for (k, list) in data::EXCLUDES_DATA.iter() {
            for pat in list {
                compile_any(pat).expect(&format!("EXCLUDES_DATA[{}] pattern {:?} failed", k, pat));
            }
        }
        for pat in data::BAD_SEMI_PHRASES.iter() {
            compile_any(pat).expect(&format!("BAD_SEMI_PHRASES pattern {:?} failed", pat));
        }
        for pat in data::BAD_PHRASES.iter() {
            compile_any(pat).expect(&format!("BAD_PHRASES pattern {:?} failed", pat));
        }
    }
}