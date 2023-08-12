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

pub mod test_util {
    // use lazy_static::lazy_static;
    // use sqlx::{Pool, Postgres};
    use std::sync::Once;

    // use crate::app::storage::postgres::create_pool;

    static INIT: Once = Once::new();

    pub fn init() {
        INIT.call_once(|| {
            dotenvy::dotenv().expect("error loading environment variables");
            let _ = env_logger::builder().is_test(true).try_init();
        });
    }

    // lazy_static! {
    //     static ref TEST_PG_POOL: Pool<Postgres> = {
    //         tokio::runtime::Runtime::new()
    //             .unwrap()
    //             .block_on(async { create_pool(100).await.expect("error initializing pool") })
    //     };
    // }
}
