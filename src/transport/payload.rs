//!
//! Payload Module
//!
use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use iota_conversion::trytes_converter::{to_string as trytes_to_string, to_trytes};
use iota_streams::{
    app_channels::api::tangle::DefaultTW, core::tbits::Tbits, protobuf3::types::Trytes,
};
use serde::{de::DeserializeOwned, Serialize};
use std::str::FromStr;

///
/// Payload
///
pub trait PacketPayload {
    ///
    /// Return the public payload data
    ///
    fn public_data(&self) -> &Trytes<DefaultTW>;
    ///
    /// Return the masked payload data
    ///
    fn masked_data(&self) -> &Trytes<DefaultTW>;
}

///
/// Packet
///
pub struct Payload(Trytes<DefaultTW>, Trytes<DefaultTW>);

impl Payload {
    ///
    /// Unwrap Data
    ///
    pub fn unwrap_data<T>(data: &Trytes<DefaultTW>) -> anyhow::Result<T>
    where
        T: DeserializeOwned,
    {
        let raw = trytes_to_string(&data.to_string())
            .map_err(|_| anyhow::anyhow!("Error on convert payload from trytes"))?;
        let decode_data = decode_config(&raw, URL_SAFE_NO_PAD)?;
        Ok(serde_json::from_slice(&decode_data)?)
    }
}

impl PacketPayload for Payload {
    fn public_data(&self) -> &Trytes<DefaultTW> {
        &self.0
    }
    fn masked_data(&self) -> &Trytes<DefaultTW> {
        &self.1
    }
}

///
/// Payload Builder
///
pub struct PayloadBuilder {
    p_data: String,
    m_data: String,
}

impl PayloadBuilder {
    ///
    /// Create Instance
    ///
    pub fn new() -> Self {
        PayloadBuilder {
            p_data: String::new(),
            m_data: String::new(),
        }
    }

    ///
    /// Public Data
    ///
    pub fn public<T>(&mut self, data: &T) -> &mut Self
    where
        T: Serialize,
    {
        let json_payload = serde_json::to_string(data).unwrap();
        self.p_data = to_trytes(&encode_config(&json_payload, URL_SAFE_NO_PAD)).unwrap();
        self
    }

    ///
    /// Public Data
    ///
    pub fn masked<T>(&mut self, data: &T) -> &mut Self
    where
        T: Serialize,
    {
        let json_payload = serde_json::to_string(data).unwrap();
        self.m_data = to_trytes(&encode_config(&json_payload, URL_SAFE_NO_PAD)).unwrap();
        self
    }

    ///
    /// Build
    ///
    pub fn build(&self) -> Payload {
        Payload(
            Trytes(Tbits::from_str(&self.p_data).unwrap()),
            Trytes(Tbits::from_str(&self.m_data).unwrap()),
        )
    }
}
