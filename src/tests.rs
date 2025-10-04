#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use crate::lang::{en::En, ru::Ru};
    use crate::{Censor, CensorLang};
    use pretty_assertions::assert_eq;

    #[test]
    fn basic_usage_example() {
        let censor = Censor::new(CensorLang::En).unwrap(); // make Censor from default language providers
        let ru_censor = Censor::from(Ru::new()).unwrap(); // or make it from your custom language provider

        let line = "dumb ass";
        let res = censor.clean_line(line);

        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);
        assert_eq!(res.bad_phrases_count, 0);
        assert!(res.detected_bad_words.iter().map(|b| b.as_ref()).eq(["dumb", "ass"]));
    }
    #[test]
    fn basic_usage_example__old() {
        let censor = Censor::new(CensorLang::En).unwrap();
        let line = "dumb ass";
        let res = censor.clean_line(line);

        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);
        assert_eq!(res.bad_phrases_count, 0);
        assert!(res.detected_bad_words.iter().map(|b| b.as_ref()).eq(["dumb", "ass"]));
    }

    #[test]
    fn clean_line_idempotency() {
        let censor = Censor::from(En::new()).unwrap();
        let line = "dumb ass";

        let result1 = censor.clean_line(line);
        let result2 = censor.clean_line(&result1.line);
        assert_eq!(result1.line, result2.line);
    }

    #[test]
    fn threaded_test() {
        let censor = Arc::new(Censor::from(En::new()).unwrap()); // or Arc::new(Censor::new(Arc::new(en))?) if you switched to Arc<L>

        let text = "dumb ass";
        let expected_words = ["dumb", "ass"];

        thread::scope(|s| {
            for _ in 0..16 {
                let c = censor.clone();
                s.spawn(move || {
                    let res = c.clean_line(text);
                    assert_eq!(res.line, format!("{0} {0}", c.data.beep));
                    assert_eq!(res.bad_words_count, 2);
                    assert_eq!(res.bad_phrases_count, 0);
                    assert!(res.detected_bad_words.iter().map(|b| b.as_ref()).eq(expected_words));
                });
            }
        });
    }

    #[test]
    fn ru_test() {
        let censor = Censor::from(Ru::new()).unwrap();

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
        let mut censor = Censor::from(En::new()).unwrap();

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
        let censor = Censor::from(Ru::new()).unwrap();
        censor.precompile_all_patterns();
    }

    #[test]
    fn compile_all_en_patterns() {
        let censor = Censor::from(Ru::new()).unwrap();
        censor.precompile_all_patterns();
    }
}