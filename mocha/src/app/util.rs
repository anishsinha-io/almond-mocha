pub mod rng {

    use rand::{distributions::Alphanumeric, Rng};

    #[inline]
    pub fn random_string(len: usize) -> String {
        rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(len)
            .map(char::from)
            .collect()
    }
}

pub mod posts {
    // TODO: Actually implement this
    pub fn read_time(text: &str) -> i64 {
        7
    }
}

pub mod time {
    use std::time::{SystemTime, UNIX_EPOCH};

    #[inline(always)]
    pub fn now() -> usize {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
    }
}

/// Everything in this module is only used in tests so it's alright if we annotate things with
/// #[allow(unused)] because they are not used in the app but are necessary in tests
#[cfg(test)]
pub mod test_util {
    use std::sync::Once;

    #[allow(unused)]
    static INIT: Once = Once::new();

    #[allow(unused)]
    pub fn init() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("error loading environment variables");
            let _ = env_logger::builder().is_test(true).try_init();
        });
    }
}
