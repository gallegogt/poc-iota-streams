//!
//! Transport Base
//!

use anyhow::{bail, ensure};
use async_trait::async_trait;
use iota_streams::app::message::TbinaryMessage;

/// Network transport abstraction.
/// Parametrized by the type of message links.
/// Message link is used to identify/locate a message (eg. like URL for HTTP).
#[async_trait]
pub trait AsyncTransport<TW, F, Link>
where
    F: Send + Sync + 'static,
    Link: Send + Sync,
    TW: Send + Sync + 'static,
{
    type SendOptions;
    type SendOutput;
    type RecvOptions;

    /// Send a message with explicit options.
    async fn send_message_with_options(
        &mut self,
        msg: &TbinaryMessage<TW, F, Link>,
        opt: Self::SendOptions,
    ) -> anyhow::Result<Self::SendOutput>;

    /// Send a message with default options.
    async fn send_message(
        &mut self,
        msg: &TbinaryMessage<TW, F, Link>,
    ) -> anyhow::Result<Self::SendOutput>
    where
        Self::SendOptions: Default + Send,
    {
        self.send_message_with_options(msg, Self::SendOptions::default())
            .await
    }

    /// Receive messages with explicit options.
    async fn recv_messages_with_options(
        &mut self,
        link: &Link,
        opt: Self::RecvOptions,
    ) -> anyhow::Result<Vec<TbinaryMessage<TW, F, Link>>>;

    /// Receive messages with explicit options.
    async fn recv_message_with_options(
        &mut self,
        link: &Link,
        opt: Self::RecvOptions,
    ) -> anyhow::Result<Option<TbinaryMessage<TW, F, Link>>>
    where
        Self::RecvOptions: Default + Send,
    {
        let mut msgs = self.recv_messages_with_options(link, opt).await?;
        if let Some(msg) = msgs.pop() {
            ensure!(msgs.is_empty(), "More than one message found.");
            Ok(Some(msg))
        } else {
            bail!("Message not found.");
        }
    }

    /// Receive messages with default options.
    async fn recv_messages(
        &mut self,
        link: &Link,
    ) -> anyhow::Result<Vec<TbinaryMessage<TW, F, Link>>>
    where
        Self::RecvOptions: Default + Send,
    {
        self.recv_messages_with_options(link, Self::RecvOptions::default())
            .await
    }

    /// Receive a message with default options.
    async fn recv_message(
        &mut self,
        link: &Link,
    ) -> anyhow::Result<Option<TbinaryMessage<TW, F, Link>>>
    where
        Self::RecvOptions: Default + Send,
    {
        self.recv_message_with_options(link, Self::RecvOptions::default())
            .await
    }
}
