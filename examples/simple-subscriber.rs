//!
//! Simple Subscriber
//!
//! How to run this example:
//!
//! ```bash
//!   cargo run --example simple-subscriber --release -- --seed <SEED> --channel <CHANNEL ADDRESS>
//!   --announcement_tag <ANNOUNCEMENT TAG>
//! ```
use iota_streams::app_channels::{
    api::tangle::{Address, Subscriber},
    message,
};

use clap::{App, Arg};
use poc::{
    sample::print_message_payload,
    transport::{recv_message, recv_messages},
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
                .long("announcement_tag")
                .takes_value(true)
                .required(true)
                .help("Stream Annuncement Tag"),
        )
        .get_matches();

    let api_url = matches
        .value_of("url")
        .unwrap_or("https://nodes.comnet.thetangle.org:443");
    let seed = matches.value_of("seed").unwrap();
    let channel_address = matches.value_of("channel_address").unwrap();
    let announcement_tag = matches.value_of("announcement_tag").unwrap();

    // Initialize the IOTA Client
    //
    let mut client = iota::Client::new(api_url).unwrap();

    // Create subscriber
    //
    let mut subscriber = Subscriber::new(seed, true);

    println!("Channel Address={}", channel_address);
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
        if preparsed.check_content_type(message::announce::TYPE) {
            subscriber
                .unwrap_announcement(preparsed)
                .map_err(|_| anyhow::anyhow!("Error unwraping the announcement message"))?;
        }
    }

    // Receive all published messages
    //
    let msg_list = recv_messages(&mut client, &announcement_link).await?;

    println!("\nReceive {} Messages", msg_list.len());

    for (_idx, msg) in msg_list.iter().enumerate() {
        let preparsed = msg
            .parse_header()
            .map_err(|_| anyhow::anyhow!("Error parsing the message header"))?;

        if preparsed.check_content_type(message::signed_packet::TYPE) {
            match subscriber.unwrap_signed_packet(preparsed.clone()) {
                Ok((unwrapped_public, unwrapped_masked)) => {
                    print_message_payload("Signed", unwrapped_public, unwrapped_masked);
                }
                Err(e) => println!("Signed Packet Error: {}", e),
            }
            continue;
        }
    }

    Ok(())
}
