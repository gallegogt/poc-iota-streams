//!
//! PoC Lib
//!
pub mod transport;

pub mod sample {
    use chrono::{Local, NaiveDateTime};
    use rand::Rng;
    use serde::{Deserialize, Serialize};

    ///
    /// Simple Streams Data
    ///
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamsData {
        ts: NaiveDateTime,
        data: String,
    }

    impl StreamsData {
        pub fn new<S>(data: S) -> Self
        where
            S: Into<String>,
        {
            StreamsData {
                ts: Local::now().naive_utc(),
                data: data.into(),
            }
        }
    }

    impl Default for StreamsData {
        fn default() -> Self {
            let mut rng = rand::thread_rng();
            StreamsData::new(make_random_data(rng.gen_range(10, 200)))
        }
    }

    ///
    /// Generate Random Data
    ///
    pub(crate) fn make_random_data(len: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789)(*&^%$#@!~;,.";
        let mut rng = rand::thread_rng();

        let password: String = (0..len)
            .map(|_| {
                let idx = rng.gen_range(0, CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();
        password
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
