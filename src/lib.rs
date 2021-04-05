//!
//! PoC Lib
//!
pub mod payload;
pub mod transport;

pub mod sample {
    use crate::payload::json::Payload;
    use chrono::{Local, NaiveDateTime};
    use crypto::hashes::{blake2b, Digest};
    use iota_streams::{app_channels::api::tangle::Address, ddml::types::Bytes};
    use rand::{Rng, distributions::Uniform};
    use serde::{Deserialize, Serialize};

    ///
    /// Simple Streams Data
    ///
    #[derive(Debug, Serialize, Deserialize)]
    pub struct StreamsData {
        /// Current Timestap
        ts: NaiveDateTime,
        /// Sample description
        desc: String,
        /// Temperature
        temperature: f32,
        /// Pressure
        pressure: f32,
    }

    impl StreamsData {
        pub fn new<S>(desc: S, temp: f32, pressure: f32) -> Self
        where
            S: Into<String>,
        {
            StreamsData {
                ts: Local::now().naive_utc(),
                desc: desc.into(),
                temperature: temp,
                pressure: pressure,
            }
        }
    }

    impl Default for StreamsData {
        fn default() -> Self {
            let mut rng = rand::thread_rng();
            StreamsData::new(
                make_random_data(rng.gen_range(10, 50)),
                rng.gen_range(-10.0, 1.3e3),
                rng.gen_range(10.0, 1.3e5),
            )
        }
    }

    ///
    /// Generate Random Data
    ///
    pub(crate) fn make_random_data(len: usize) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                                abcdefghijklmnopqrstuvwxyz\
                                0123456789 ;,.";
        rand::thread_rng()
            .sample_iter(Uniform::new(0, CHARSET.len()))
            .take(len)
            .map(|i| CHARSET[i] as char)
            .collect()
    }
    ///
    /// Generate Random Seed
    ///
    pub fn make_random_seed() -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ9";
        rand::thread_rng()
            .sample_iter(Uniform::new(0, CHARSET.len()))
            .take(81)
            .map(|i| CHARSET[i] as char)
            .collect()
    }

    /// Print message payload
    ///
    pub fn print_message_payload<T>(prefix: T, public: &Bytes, masked: &Bytes)
    where
        T: Into<String>,
    {
        let p_data: Option<StreamsData> = Payload::unwrap_data(&public).unwrap();
        let m_data: Option<StreamsData> = Payload::unwrap_data(&masked).unwrap();
        let pfx = prefix.into();

        match p_data {
            Some(d) => println!("\n {} Public Packet: \n \t{:?}\n", pfx, d),
            None => {}
        }
        match m_data {
            Some(d) => println!("\n {} Masked Packet: \n \t{:?}\n", pfx, d),
            None => {}
        }
    }

    pub fn get_message_index(link: &Address) -> String {
        let arr = [link.appinst.as_ref(), link.msgid.as_ref()].concat();
        let hs = blake2b::Blake2b256::digest(&arr);
        hex::encode(&hs)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
