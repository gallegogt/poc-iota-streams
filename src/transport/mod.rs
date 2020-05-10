//!
//! Transport Module
//!
mod base;
mod tangle;

use failure::Fallible;
use iota_conversion::trytes_converter::to_trytes;
use iota_streams::{
    app_channels::api::tangle::{Address, DefaultF, DefaultTW, Message},
    core::tbits::Tbits,
    protobuf3::types::Trytes,
};
use std::str::FromStr;
///
/// AsyncTransport
///
pub trait AsyncTransport: base::AsyncTransport<DefaultTW, DefaultF, Address> {}
impl<T> AsyncTransport for T where T: base::AsyncTransport<DefaultTW, DefaultF, Address> {}

///
/// Payload
///
pub trait TPacket {
    ///
    /// Return the public payload data
    ///
    fn public_data(&self) -> Trytes<DefaultTW>;
    ///
    /// Return the masked payload data
    ///
    fn masked_data(&self) -> Trytes<DefaultTW>;
}

///
/// Packet
///
pub struct Payload(Tbits<DefaultTW>, Tbits<DefaultTW>);

impl TPacket for Payload {
    fn public_data(&self) -> Trytes<DefaultTW> {
        Trytes(self.0.clone())
    }
    fn masked_data(&self) -> Trytes<DefaultTW> {
        Trytes(self.1.clone())
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
    pub fn public_data<'a>(&mut self, data: &'a str) -> &mut Self {
        self.p_data = to_trytes(data).unwrap();
        self
    }

    ///
    /// Masked Data
    ///
    pub fn masked_data<'a>(&mut self, data: &'a str) -> &mut Self {
        self.p_data = to_trytes(data).unwrap();
        self
    }

    ///
    /// Build
    ///
    pub fn build(&self) -> Payload {
        Payload(
            Tbits::from_str(&self.p_data).unwrap(),
            Tbits::from_str(&self.m_data).unwrap(),
        )
    }
}

///
/// Read all messages in the thangle for this address
///
pub async fn recv_messages<T>(transport: &mut T, addr: &Address) -> Fallible<Vec<Message>>
where
    T: AsyncTransport,
    <T>::RecvOptions: Copy + Default + Send,
{
    transport
        .recv_messages_with_options(addr, T::RecvOptions::default())
        .await
}

///
/// Read one message
///
pub async fn recv_message<T>(transport: &mut T, addr: &Address) -> Fallible<Option<Message>>
where
    T: AsyncTransport + Send,
    <T>::RecvOptions: Copy + Default + Send,
{
    transport.recv_message(addr).await
}

///
/// Send a message
///
pub async fn send_message<T>(transport: &mut T, message: &Message) -> Fallible<()>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
{
    transport.send_message(message).await?;
    Ok(())
}
