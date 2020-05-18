//!
//! Transport Module
//!
mod base;
pub mod payload;
mod tangle;

use iota_streams::app_channels::api::tangle::{Address, DefaultF, DefaultTW, Message};
pub use tangle::IotaTransport;

///
/// AsyncTransport
///
pub trait AsyncTransport: base::AsyncTransport<DefaultTW, DefaultF, Address> {}
impl<T> AsyncTransport for T where T: base::AsyncTransport<DefaultTW, DefaultF, Address> {}

///
/// Read all messages in the thangle for this address
///
pub async fn recv_messages<T>(transport: &mut T, addr: &Address) -> anyhow::Result<Vec<Message>>
where
    T: AsyncTransport,
    <T>::RecvOptions: Copy + Default + Send,
{
    let messages = transport
        .recv_messages_with_options(addr, T::RecvOptions::default())
        .await?;
    Ok(messages)
}

///
/// Read one message
///
pub async fn recv_message<T>(transport: &mut T, addr: &Address) -> anyhow::Result<Option<Message>>
where
    T: AsyncTransport + Send,
    <T>::RecvOptions: Copy + Default + Send,
{
    transport.recv_message(addr).await
}

///
/// Send a message
///
pub async fn send_message<T>(transport: &mut T, message: &Message) -> anyhow::Result<()>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
{
    match transport.send_message(message).await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
