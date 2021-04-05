//!
//! Simple IOTA Streams Author
//!
//! * This example sends all linked signed messages to the announce and
//!   only 2 ^ mss_height signed messages will be sent
//!
//! How run this example:
//!
//! ```bash
//!   cargo run --example e01-author --release -- --seed <SEED> [--mss-height 3]
//! ```
//!
use clap::{App, Arg};
use iota_streams::{
    app::transport::tangle::PAYLOAD_BYTES,
    app_channels::api::tangle::{Address, Author, Transport},
};
// use std::cell::RefCell;

use poc::{
    payload::{json::PayloadBuilder, PacketPayload},
    sample::{StreamsData, make_random_seed, get_message_index},
    transport::build_transport,
};

use std::{time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rseed = make_random_seed();
    let matches = App::new("Simple IOTA Streams Author")
        .version("1.0")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .default_value(&rseed)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .short("p")
                .long("url")
                .takes_value(true)
                .default_value("https://api.lb-0.testnet.chrysalis2.com")
                .help("The Tangle Url, Default: https://nodes.comnet.thetangle.org:443"),
        )
        .arg(
            Arg::with_name("mss_height")
                .short("m")
                .long("mss-height")
                .takes_value(true)
                .help("Merkle Tree Signature Height, Default: 3"),
        )
        .arg(
            Arg::with_name("encoding")
                .long("encoding")
                .default_value("utf-8")
                .help("Encoding, Default UTF-8"),
        )
        .get_matches();

    let api_url = matches
        .value_of("url")
        .unwrap_or("https://nodes.comnet.thetangle.org:443");

    let seed = matches.value_of("seed").unwrap();
    let encoding = matches.value_of("encoding").unwrap();
    let mss_height: usize = matches
        .value_of("mss_height")
        .unwrap_or("3")
        .parse()
        .unwrap_or(3);

    let transport = build_transport(api_url, 9);

    // Create the author
    //
    let mut author = Author::new(seed, encoding, PAYLOAD_BYTES, false, transport.clone());

    println!("\rChannel Address (Copy this Address for the Subscribers):");
    println!("\t{:?}\n", author.channel_address().unwrap());

    // Creare Announcement Tag
    //
    let announcement_link = {
        let msg = &author
            .send_announce()
            .await
            .map_err(|_| anyhow::anyhow!("Error creating announce message"))?;

        println!("Announcement Message Tag:");
        println!("\t{}\n", msg.msgid);

        msg.clone()
    };

    // Posible numbers of messages to sign before change the key
    //
    let remaining_sk = 2_u32.pow(mss_height as u32);
    let mut remaining_signed_messages = remaining_sk;

    let mut linked_ = announcement_link.clone();
    // Announcement Link
    //
    loop {
        if remaining_signed_messages <= 0 {
            break;
        } else {
            remaining_signed_messages = remaining_signed_messages - 1;
        }
        let data = StreamsData::default();
        println!("DATA={:?}", &data);
        let _link_signed = send_tagged_data(
            &mut author,
            &linked_,
            PayloadBuilder::new().masked(&data)?.build(),
        )
        .await
        .unwrap();

        linked_ = _link_signed;
        tokio::time::sleep(Duration::from_secs(10)).await;
    }

    Ok(())
}

///
/// Send a signed message
///
async fn send_tagged_data<'a, T, P>(
    author: &mut Author<T>,
    addrs: &Address,
    payload: P,
) -> anyhow::Result<Address>
where
    T: Transport + Clone,
    P: PacketPayload,
{
    let signed_packet_link = {
        let (msg, seq) = author
            .send_tagged_packet(&addrs, &payload.public_data(), &payload.masked_data())
            .await
            .map_err(|_| anyhow::anyhow!("Error to create signed packet"))?;
        println!("\tTagged Message ID={}", msg.msgid);
        println!("\tSeq={:?}", seq);
        println!("\tMessageIndex={:?}", get_message_index(&msg));
        msg
    };
    Ok(signed_packet_link)
}
