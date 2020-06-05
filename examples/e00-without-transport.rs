//!
//! Author - Subscriber Without Transport
//!
//! How run this example:
//!
//! ```bash
//!   cargo run --example without-transport --release -- --seed-author <SEED> --seed-subscriber <SEED> [--mss_height 3]
//! ```
//!
use clap::{App, Arg};
use iota_streams::app_channels::{
    api::tangle::{Address, Author, Message, Subscriber},
    message,
};
use poc::{
    payload::{json::PayloadBuilder, PacketPayload},
    sample::{print_message_payload, StreamsData},
};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Author Of Linked Messages")
        .version("1.0")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed-author")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("seed_subscriber")
                .short("b")
                .long("seed-subscriber")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mss_height")
                .short("m")
                .long("mss_height")
                .takes_value(true)
                .help("Merkle Tree Signature Height, Default: 3"),
        )
        .arg(
            Arg::with_name("use_ntru")
                .short("u")
                .long("use-ntru")
                .help("Stream Subscriber use NTRU"),
        )
        .get_matches();

    let seed = matches.value_of("seed").unwrap();
    let seed_sub = matches.value_of("seed_subscriber").unwrap();
    let mss_height: usize = matches
        .value_of("mss_height")
        .unwrap_or("3")
        .parse()
        .unwrap_or(3);
    let use_ntru = matches.is_present("use_ntru");

    // Create the author
    //
    let mut author = Author::new(seed, mss_height, use_ntru);
    // Create subscriber
    //
    let mut subscriber = Subscriber::new(seed_sub, use_ntru);

    println!("\rChannel Address (Copy this Address for the Subscribers):");
    println!("\t{}\n", author.channel_address());

    // Creare Announcement Tag
    //
    let (mut link_addr, announce_message) = {
        let msg = &author
            .announce()
            .map_err(|_| anyhow::anyhow!("Error creating announce message"))?;

        println!("Announcement Message Tag:");
        println!("\t{}\n", msg.link.msgid);

        (msg.link.clone(), msg.clone())
    };

    // Subscriber read the announce message
    //
    {
        let preparsed = announce_message
            .parse_header()
            .map_err(|_| anyhow::anyhow!("Error parsing the announce header"))?;
        if preparsed.check_content_type(message::announce::TYPE) {
            subscriber
                .unwrap_announcement(preparsed)
                .map_err(|_| anyhow::anyhow!("Error unwraping the announcement message"))?;
        }
    }

    // Posible numbers of messages to sign before change the key
    //
    let remaining_sk = 2_u32.pow(mss_height as u32);
    let mut remaining_signed_messages = remaining_sk;

    loop {
        if remaining_signed_messages <= 0 {
            let chk_message = author
                .change_key(&link_addr)
                .map_err(|_| anyhow::anyhow!("Error changing the key"))?;

            link_addr = chk_message.link.clone();
            remaining_signed_messages = remaining_sk;

            println!("Change Key Message Tag:");
            println!("\t{}\n", chk_message.link.msgid);
            println!("Change Key Message Address:");
            println!("\t{}\n", chk_message.link.appinst.to_string());

            // Subscriber read the new Change Key Message
            //
            subscriber_read_change_key_message(&mut subscriber, &chk_message).unwrap();
        } else {
            remaining_signed_messages = remaining_signed_messages - 1;
        }

        let (link_addr_t, message) = send_signed_data(
            &mut author,
            &link_addr,
            PayloadBuilder::new()
                .masked(&StreamsData::default())?
                .build(),
        )
        .unwrap();
        link_addr = link_addr_t.clone();

        // The subscriber read the signed message
        //
        subscriber_read_signed_message(&mut subscriber, &message).unwrap();

        tokio::time::delay_for(Duration::from_secs(1)).await;
    }
}

///
/// Read Change key Message
///
fn subscriber_read_change_key_message(
    sub: &mut Subscriber,
    message: &Message,
) -> anyhow::Result<()> {
    let preparsed = message
        .parse_header()
        .map_err(|_| anyhow::anyhow!("Error parsing the message header"))?;

    if preparsed.check_content_type(message::change_key::TYPE) {
        sub.unwrap_change_key(preparsed.clone()).unwrap();
    }

    Ok(())
}

///
/// Read Change key Message
///
fn subscriber_read_signed_message(sub: &mut Subscriber, message: &Message) -> anyhow::Result<()> {
    let preparsed = message
        .parse_header()
        .map_err(|_| anyhow::anyhow!("Error parsing the message header"))?;

    println!("Link: {}", preparsed.header.link.msgid);

    if preparsed.check_content_type(message::signed_packet::TYPE) {
        match sub.unwrap_signed_packet(preparsed.clone()) {
            Ok((unwrapped_public, unwrapped_masked)) => {
                print_message_payload("Signed", unwrapped_public, unwrapped_masked);
            }
            Err(e) => println!("Signed Packet Error: {}", e),
        }
    }

    Ok(())
}
///
/// Send a signed message
///
fn send_signed_data<'a, P>(
    author: &mut Author,
    addrs: &Address,
    payload: P,
) -> anyhow::Result<(Address, Message)>
where
    P: PacketPayload,
{
    let (signed_packet_link, message) = {
        let msg = author
            .sign_packet(&addrs, &payload.public_data(), &payload.masked_data())
            .map_err(|_| anyhow::anyhow!("Error to create signed packet"))?;
        (msg.link.clone(), msg.clone())
    };
    Ok((signed_packet_link, message))
}
