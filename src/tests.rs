#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::thread;
    use crate::lang::{en::En, ru::Ru};
    use crate::{Censor};
    use pretty_assertions::assert_eq;

    fn assert_send_sync<T: Send + Sync>() {}

    #[test]
    fn censor_is_send_sync() {
        assert_send_sync::<Censor<'static, En>>();
        assert_send_sync::<Censor<'static, Ru>>();
    }

    #[test]
    fn basic_usage_example() {
        let en = En {}; // create a lang provider first
        let censor = Censor::new(&en).unwrap();
        let line = "dumb ass";
        let res = censor.clean_line(line);

        assert_eq!(res.line, format!("{0} {0}", censor.data.beep));
        assert_eq!(res.bad_words_count, 2);
        assert_eq!(res.bad_phrases_count, 0);
        assert!(res.detected_bad_words.iter().map(|b| b.as_ref()).eq(["dumb", "ass"]));
    }

    #[test]
    fn clean_line_idempotency() {
        let en = En {}; // create a lang provider first
        let censor = Censor::new(&en).unwrap();
        let line = "dumb ass";

        let result1 = censor.clean_line(line);
        let result2 = censor.clean_line(&result1.line);
        assert_eq!(result1.line, result2.line);
    }

    #[test]
    fn threaded_test() {
        let en = En {};
        let censor = Arc::new(Censor::new(&en).unwrap()); // or Arc::new(Censor::new(Arc::new(en))?) if you switched to Arc<L>

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
        let ru = Ru {}; // create a lang provider first
        let censor = Censor::new(&ru).unwrap();

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
        let en = En {}; // create a lang provider first
        let mut censor = Censor::new(&en).unwrap();

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
        let ru = Ru {};
        let censor = Censor::new(&ru).unwrap();
        censor.precompile_all_patterns();
    }

    #[test]
    fn compile_all_en_patterns() {
        let en = En {};
        let censor = Censor::new(&en).unwrap();
        censor.precompile_all_patterns();
    }
}