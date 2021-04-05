//!
//! Transport Module
//!

use iota_streams::{
    app::message::HasLink as _,
    app::transport::{
        tangle::client::{Client, SendOptions},
        TransportOptions,
    },
    app_channels::api::tangle::{
        Address, Author, MessageContent, Subscriber, Transport, UnwrappedMessage,
    },
    core::prelude::Rc,
    ddml::types::Bytes,
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

pub enum FetchMessageContentType {
    SignedPacket,
    TaggedPacket,
}

pub async fn s_fetch_next_messages<T: Transport>(
    subscriber: &mut Subscriber<T>,
    msgs_types: FetchMessageContentType,
    load_data: bool,
) -> Vec<(Address, Bytes, Bytes)> {
    let mut exists = true;
    let mut messages = Vec::new();

    while exists {
        let msgs = subscriber.fetch_next_msgs().await;
        exists = msgs.len() > 0;

        if load_data {
            for msg in msgs {
                match &msg.body {
                    MessageContent::SignedPacket {
                        public_payload,
                        masked_payload,
                        ..
                    } => match msgs_types {
                        FetchMessageContentType::SignedPacket => messages.push((
                            msg.link,
                            public_payload.clone(),
                            masked_payload.clone(),
                        )),
                        _ => {}
                    },
                    MessageContent::TaggedPacket {
                        public_payload,
                        masked_payload,
                    } => match msgs_types {
                        FetchMessageContentType::TaggedPacket => messages.push((
                            msg.link,
                            public_payload.clone(),
                            masked_payload.clone(),
                        )),
                        _ => {}
                    },
                    _ => {}
                }
            }
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
        let msgs = author.fetch_next_msgs().await;
        exists = msgs.len() > 0;

        for msg in msgs {
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
