//!
//! Payload Module
//!
use iota_streams::ddml::types::Bytes;
use serde::{de::DeserializeOwned, Serialize};
use std::marker::PhantomData;

///
/// Simple Trait to transform the payload to string using any serde serializer
///
pub trait PayloadSerializer {
    ///
    /// Transform data to String
    ///
    fn serialize_data<T: Serialize>(data: &T) -> anyhow::Result<String>;

    ///
    /// Transform data to String
    ///
    fn deserialize_data<T: DeserializeOwned>(data: &[u8]) -> anyhow::Result<T>;
}

///
/// Payload
///
pub trait PacketPayload {
    ///
    /// Return the public payload data
    ///
    fn public_data(&self) -> &Bytes;
    ///
    /// Return the masked payload data
    ///
    fn masked_data(&self) -> &Bytes;
}

///
/// Packet
///
pub struct Payload<S> {
    public: Bytes,
    masked: Bytes,
    _marker: PhantomData<S>,
}

impl<S> Payload<S>
where
    S: PayloadSerializer,
{
    ///
    /// Unwrap JSON Data
    ///
    pub fn unwrap_data<T>(data: &Bytes) -> anyhow::Result<Option<T>>
    where
        T: DeserializeOwned,
    {
        let data_str = data.to_string();
        if data_str.len() == 0 {
            return Ok(None);
        }
        let raw = String::from_utf8(data.0.clone()).unwrap();
        // let decode_data = decode_config(&raw, URL_SAFE)?;
        Ok(Some(S::deserialize_data(&raw.as_bytes())?))
    }
}

impl<S> PacketPayload for Payload<S> {
    fn public_data(&self) -> &Bytes {
        &self.public
    }
    fn masked_data(&self) -> &Bytes {
        &self.masked
    }
}

///
/// Payload Builder
///
pub struct PayloadBuilder<S> {
    p_data: String,
    m_data: String,
    _marker: PhantomData<S>,
}

impl<S> PayloadBuilder<S>
where
    S: PayloadSerializer,
{
    ///
    /// Create Instance
    ///
    pub fn new() -> Self {
        PayloadBuilder {
            p_data: String::new(),
            m_data: String::new(),
            _marker: PhantomData,
        }
    }

    ///
    /// Public Data
    ///
    pub fn public<T>(&mut self, data: &T) -> anyhow::Result<&mut Self>
    where
        T: Serialize,
    {
        self.p_data = S::serialize_data(data)?;
        // self.p_data = encode_config(&payload_str, URL_SAFE);
        Ok(self)
    }

    ///
    /// Public Data
    ///
    pub fn masked<T>(&mut self, data: &T) -> anyhow::Result<&mut Self>
    where
        T: Serialize,
    {
        self.p_data = S::serialize_data(data)?;
        // self.m_data = encode_config(&payload_str, URL_SAFE);
        Ok(self)
    }

    ///
    /// Build
    ///
    pub fn build(&self) -> Payload<S> {
        Payload {
            public: Bytes(self.p_data.as_bytes().to_vec()),
            masked: Bytes(self.m_data.as_bytes().to_vec()),
            _marker: PhantomData,
        }
    }
}

pub mod json {
    //!
    //! JSON Payload Serialization module
    //!
    use super::PayloadSerializer;
    use serde::{de::DeserializeOwned, Serialize};

    ///
    /// Implementation of JSON Serialize
    ///
    pub struct JsonSerializer;

    impl PayloadSerializer for JsonSerializer {
        fn serialize_data<T: Serialize>(data: &T) -> anyhow::Result<String> {
            serde_json::to_string(data).map_err(|e| anyhow::anyhow!("{:#?}", e))
        }

        fn deserialize_data<T: DeserializeOwned>(data: &[u8]) -> anyhow::Result<T> {
            serde_json::from_slice(data).map_err(|e| anyhow::anyhow!("{:#?}", e))
        }
    }

    /// Payload JSON
    ///
    pub type Payload = super::Payload<JsonSerializer>;

    /// Payload Builder in Json Format
    ///
    pub type PayloadBuilder = super::PayloadBuilder<JsonSerializer>;
}
