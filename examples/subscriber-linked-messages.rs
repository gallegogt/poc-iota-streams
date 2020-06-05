//!
//! Subscriber for Linked Messages
//!
//! How to run this example:
//!
//! ```bash
//!   cargo run --example subscriber-linked-messages --release -- --seed <SEED> --channel <CHANNEL ADDRESS>
//!   --announcement_tag <ANNOUNCEMENT TAG>
//! ```
use iota_streams::app_channels::{
    api::tangle::{Address, Subscriber},
    message,
};

use clap::{App, Arg};
use poc::{
    sample::print_message_payload,
    transport::{recv_message, recv_messages, IotaTransport},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Simple IOTA Streams Subscriber")
        .version("1.0")
        .arg(
            Arg::with_name("seed")
                .short("s")
                .long("seed")
                .required(true)
                .takes_value(true),
        )
        .arg(
            Arg::with_name("url")
                .short("p")
                .long("url")
                .takes_value(true)
                .default_value("https://nodes.comnet.thetangle.org:443")
                .help("The Tangle URL, Default: https://nodes.comnet.thetangle.org:443"),
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
            Arg::with_name("use_ntru")
                .short("u")
                .long("use-ntru")
                .help("Stream Subscriber use NTRU"),
        )
        .get_matches();

    let api_url = matches
        .value_of("url")
        .unwrap_or("https://nodes.comnet.thetangle.org:443");
    let seed = matches.value_of("seed").unwrap();
    let channel_address = matches.value_of("channel_address").unwrap();
    let announcement_tag = matches.value_of("announcement_tag").unwrap();
    let use_ntru = matches.is_present("use_ntru");

    // Initialize the IOTA Client
    //
    let mut client = IotaTransport::add_node(api_url).unwrap();

    // Create subscriber
    //
    let mut subscriber = Subscriber::new(seed, use_ntru);

    println!("\n\nChannel Address={}", channel_address);
    println!("Announcement Tag ID={}", announcement_tag);

    // Create the announcement Link
    //
    let announcement_link = Address::from_str(&channel_address, &announcement_tag).unwrap();

    // Lookup message from the tangle
    //
    let msg_announce = recv_message(&mut client, &announcement_link).await?;

    if let Some(msg) = msg_announce {
        let preparsed = msg
            .parse_header()
            .map_err(|_| anyhow::anyhow!("Error parsing the announce header"))?;

        println!("Announce Link: {}", preparsed.header.link.msgid);

        if preparsed.check_content_type(message::announce::TYPE) {
            subscriber
                .unwrap_announcement(preparsed)
                .map_err(|_| anyhow::anyhow!("Error unwraping the announcement message"))?;
        }
    } else {
        println!("No Announce Message");
        std::process::exit(1);
    }

    if use_ntru {
        print!("Subscribe to the channel ..");
        let _msg = subscriber
            .subscribe(&announcement_link)
            .map_err(|_| anyhow::anyhow!("Error on subscriber link"))?;
        print!("[OK]\n\r");
    }

    // Receive all published messages
    //
    let msg_list = recv_messages(&mut client, &announcement_link).await?;

    println!("\nReceive {} Messages", msg_list.len());

    for (_idx, msg) in msg_list.iter().enumerate() {
        let preparsed = msg
            .parse_header()
            .map_err(|_| anyhow::anyhow!("Error parsing the message header"))?;
        println!("Link: {}", preparsed.header.link.msgid);

        if preparsed.check_content_type(message::signed_packet::TYPE) {
            match subscriber.unwrap_signed_packet(preparsed.clone()) {
                Ok((unwrapped_public, unwrapped_masked)) => {
                    print_message_payload("Signed", unwrapped_public, unwrapped_masked);
                }
                Err(e) => println!("Signed Packet Error: {}", e),
            }
            continue;
        }

        if preparsed.check_content_type(message::change_key::TYPE) {
            subscriber.unwrap_change_key(preparsed.clone()).unwrap();
            continue;
        }

        if preparsed.check_content_type(message::keyload::TYPE) {
            match subscriber.unwrap_keyload(preparsed.clone()) {
                Ok(_) => {}
                Err(e) => println!("Keyload Error: {}", e),
            }
            continue;
        }
    }

    Ok(())
}
