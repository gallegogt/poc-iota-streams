//!
//! Transport Module
//!

use iota_streams::{
    app::message::HasLink as _,
    app::transport::{
        tangle::client::{Client, SendOptions},
        TransportOptions,
    },
    app_channels::api::tangle::{Address, Author, Subscriber, Transport, UnwrappedMessage},
    core::prelude::Rc,
};
use std::cell::RefCell;

pub fn build_transport<'a>(uri: &'a str, node_mwm: u8) -> Rc<RefCell<Client>> {
    let client = Client::new_from_url(&uri);

    let mut transport = Rc::new(RefCell::new(client));
    let mut send_opt = SendOptions::default();
    send_opt.min_weight_magnitude = node_mwm;
    transport.set_send_options(send_opt);

    transport
}

pub async fn s_fetch_next_messages<T: Transport>(subscriber: &mut Subscriber<T>) -> Vec<Address> {
    let mut exists = true;
    let mut messages = Vec::new();

    while exists {
        let msgs = subscriber.fetch_next_msgs();
        exists = false;

        for msg in msgs.await {
            println!("Message exists at {}... ", &msg.link.rel());
            exists = true;
            messages.push(msg.link);
        }

        if !exists {
            println!("No more messages in sequence.");
        }
    }
    messages
}

pub async fn a_fetch_next_messages<T: Transport>(author: &mut Author<T>) -> Vec<UnwrappedMessage> {
    let mut exists = true;
    let mut messages = Vec::new();

    while exists {
        let msgs = author.fetch_next_msgs();
        exists = false;

        for msg in msgs.await {
            println!("Message exists at {}... ", &msg.link.rel());
            messages.push(msg);
            exists = true;
        }

        if !exists {
            println!("No more messages in sequence.");
        }
    }

    messages
}
// ///
// /// Read all messages in the thangle for this address
// ///
// pub async fn recv_messages<T>(transport: &mut T, addr: &Address) -> anyhow::Result<Vec<Message>>
// where
//     T: AsyncTransport,
//     <T>::RecvOptions: Copy + Default + Send,
// {
//     let messages = transport
//         .recv_messages_with_options(addr, T::RecvOptions::default())
//         .await?;
//     Ok(messages)
// }
//
// ///
// /// Read one message
// ///
// pub async fn recv_message<T>(transport: &mut T, addr: &Address) -> anyhow::Result<Option<Message>>
// where
//     T: AsyncTransport + Send,
//     <T>::RecvOptions: Copy + Default + Send,
// {
//     transport.recv_message(addr).await
// }
//
// ///
// /// Send a message
// ///
// pub async fn send_message<T>(transport: &mut T, message: &Message) -> anyhow::Result<()>
// where
//     T: AsyncTransport + Send,
//     <T>::SendOptions: Copy + Default + Send,
// {
//     match transport.send_message(message).await {
//         Ok(_) => Ok(()),
//         Err(e) => Err(e),
//     }
// }
