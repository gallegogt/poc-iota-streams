//!
//! Simple Subscriber
//!
//! Fetch all message linked in the channel, published by the example e01-author
//!
//! How to run this example:
//!
//! ```bash
//!   cargo run --example e01-subscriber --release -- --seed <SEED> --channel <CHANNEL ADDRESS>
//!   --announcement_tag <ANNOUNCEMENT TAG> [--message-id <MESSAGE ID>]
//! ```
use clap::{App, Arg};
use iota_streams::{
    app::transport::tangle::PAYLOAD_BYTES,
    app_channels::api::tangle::{Address, Subscriber},
};
use poc::{
    sample::{make_random_seed, print_message_payload},
    transport::{build_transport, s_fetch_next_messages, FetchMessageContentType},
};
use regex::Regex;
use tokio::io::{stdin, AsyncBufReadExt, BufReader};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rseed = make_random_seed();
    let matches = App::new("Simple IOTA Streams Subscriber")
        .version("1.0")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .takes_value(true)
                .default_value(&rseed),
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
            Arg::with_name("channel_address")
                .short("c")
                .long("channel")
                .takes_value(true)
                .required(true)
                .help("Stream channel address"),
        )
        .arg(
            Arg::with_name("announcement_tag")
                .short("n")
                .long("announcement-tag")
                .takes_value(true)
                .required(true)
                .help("Stream Annuncement Tag"),
        )
        .arg(
            Arg::with_name("message_id")
                .long("message-id")
                .takes_value(true)
                .help("Access to message by id"),
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
    let channel_address = matches.value_of("channel_address").unwrap();
    let announcement_tag = matches.value_of("announcement_tag").unwrap();
    let message_id = matches.value_of("message_id").unwrap_or("");

    // Initialize the IOTA Client
    //
    let transport = build_transport(api_url, 9);

    // Create subscriber
    //
    let mut subscriber = Subscriber::new(
        seed,
        matches.value_of("encoding").unwrap(),
        PAYLOAD_BYTES,
        transport.clone(),
    );

    println!("Channel Address={}", channel_address);
    println!("Announcement Tag ID={}", announcement_tag);

    // Create the announcement Link
    //
    let announcement_link = Address::from_str(&channel_address, &announcement_tag).unwrap();

    // Receive all published messages
    //
    subscriber
        .receive_announcement(&announcement_link)
        .await
        .unwrap();

    println!(
        "\nSubscriber Channel Address {}",
        subscriber.channel_address().unwrap()
    );

    {
        let msg = subscriber.send_subscribe(&announcement_link).await.unwrap();
        println!(
            "Subscriber ID: {} (Copy and paste on author example) \n",
            msg.msgid
        );
        println!("SET \"{}\" \n", msg.msgid);
    };

    let mut lines = BufReader::new(stdin()).lines();

    println!(
        "Type Keyload Message ID (Example: SET \"e2feafdd5c6a72cef26ea3b2\" ) and press enter \n"
    );

    while let Some(line) = lines.next_line().await.unwrap() {
        match Regex::new("SET\\s+\"(?P<target>[[:alnum:]]{20,32}?)\"") {
            Ok(regex) => {
                if let Some(capture) = regex.captures(&line) {
                    if let Some(keyload_id) =
                        capture.name("target").and_then(|s| Some(s.as_str().trim()))
                    {
                        subscriber
                            .receive_keyload(
                                &Address::from_str(
                                    &format!("{}", announcement_link.appinst),
                                    &keyload_id,
                                )
                                .unwrap(),
                            )
                            .await
                            .unwrap();

                        println!("Received Keyload {} \n", keyload_id);
                        break;
                    }
                } else {
                    eprintln!("Missing argument keyload id...")
                }
            }
            Err(_) => println!("Try again ..."),
        }
    }

    if message_id.is_empty() {
        // Lis all data linked in the channel
        //
        let mut msg_list =
            s_fetch_next_messages(&mut subscriber, FetchMessageContentType::SignedPacket, true)
                .await;

        while msg_list.len() > 0 {
            for (idx, (msg, unwrapped_public, unwrapped_masked)) in msg_list.iter().enumerate() {
                print_message_payload(
                    format!("{}.- Signed ({})", idx, msg),
                    &unwrapped_public,
                    &unwrapped_masked,
                );
            }
            msg_list =
                s_fetch_next_messages(&mut subscriber, FetchMessageContentType::SignedPacket, true)
                    .await;
        }
    } else {
        // Get Linked Data
        //
        let _ = s_fetch_next_messages(
            &mut subscriber,
            FetchMessageContentType::SignedPacket,
            false,
        )
        .await;
        let message_link = Address::from_str(&channel_address, &message_id).unwrap();
        // Access to specific message
        //
        let (_signer_pk, uw_public, uw_masked) = subscriber
            .receive_signed_packet(&message_link)
            .await
            .unwrap();

        print_message_payload(format!("{} - Signed", message_id), &uw_public, &uw_masked);
    }

    Ok(())
}
