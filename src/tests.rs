#[cfg(test)]
mod tests {
    use crate::{Censor, CensorLang};
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_usage_example() {
        let en_censor = Censor::new(CensorLang::En).unwrap();
        let line = "dumb ass";
        let res = en_censor.clean_line(line);

        assert_eq!(res.line, format!("{0} {0}", en_censor.data.beep));
        assert_eq!(res.bad_words_count, 2);
        assert_eq!(res.bad_phrases_count, 0);
        assert_eq!(res.detected_bad_words, ["dumb", "ass"]);
    }
    #[test]
    fn ru_test() {
        let censor = Censor::new(CensorLang::Ru).unwrap();

        let line = "сykА блядб";
        let res = censor.clean_line(line);
        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);

        let line = "какого хYя";
        let res = censor.clean_line(line);
        assert_eq!(res.line, format!("какого {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 1);
    }

    #[test]
    fn en_test() {
        let mut censor = Censor::new(CensorLang::En).unwrap();

        let line = "dumb ass";
        let res = censor.clean_line(line);
        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);

        let line = "ok what the fuck";
        censor.data.set_beep("###");
        let res = censor.clean_line(line);
        assert_eq!(res.line, format!("ok what the {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 1);
    }

    #[test]
    fn compile_all_ru_patterns() {
        let censor = Censor::new(CensorLang::Ru).unwrap();
        censor.precompile_all_patterns();
    }

    #[test]
    fn compile_all_en_patterns() {
        let censor = Censor::new(CensorLang::En).unwrap();
        censor.precompile_all_patterns();
    }
}