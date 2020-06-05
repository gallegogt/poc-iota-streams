//!
//! Simple IOTA Streams Author
//!
//! * This example sends all linked signed messages to the announce
//!
//! How run this example:
//!
//! ```bash
//!   cargo run --example e02-author-with-changekey --release -- --seed <SEED> [--mss-height 3]
//! ```
//!
use clap::{App, Arg};
use iota_streams::app_channels::api::tangle::{Address, Author};
use poc::{
    payload::{json::PayloadBuilder, PacketPayload},
    sample::StreamsData,
    transport::{send_message, AsyncTransport, IotaTransport},
};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Simple IOTA Streams Author")
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
    let mss_height: usize = matches
        .value_of("mss_height")
        .unwrap_or("3")
        .parse()
        .unwrap_or(3);

    let use_ntru = matches.is_present("use_ntru");
    // Initialize The IOTA Client
    //
    let mut client = IotaTransport::add_node(api_url).unwrap();

    // Create the author
    //
    let mut author = Author::new(seed, mss_height, use_ntru);

    println!("\rChannel Address (Copy this Address for the Subscribers):");
    println!("\t{}\n", author.channel_address());

    // Creare Announcement Tag
    //
    let (announcement_address, announcement_tag) = {
        let msg = &author
            .announce()
            .map_err(|_| anyhow::anyhow!("Error creating announce message"))?;

        println!("Announcement Message Tag:");
        println!("\t{}\n", msg.link.msgid);

        send_message(&mut client, msg).await.unwrap();

        (msg.link.appinst.to_string(), msg.link.msgid.to_string())
    };

    // Posible numbers of messages to sign before change the key
    //
    let remaining_sk = 2_u32.pow(mss_height as u32);
    let mut remaining_signed_messages = remaining_sk;

    // Announcement Link
    //
    let announcement_link = Address::from_str(&announcement_address, &announcement_tag).unwrap();
    loop {
        if remaining_signed_messages <= 0 {
            let chk_message = author
                .change_key(&announcement_link)
                .map_err(|_| anyhow::anyhow!("Error changing the key"))?;

            // send change key message
            //
            send_message(&mut client, &chk_message).await.unwrap();
            remaining_signed_messages = remaining_sk;

            println!("Change Key Message Tag:");
            println!("\t{}\n", chk_message.link.msgid);
            println!("Change Key Message Address:");
            println!("\t{}\n", chk_message.link.appinst.to_string());
        } else {
            remaining_signed_messages = remaining_signed_messages - 1;
        }

        let _link_signed = send_signed_data(
            &mut client,
            &mut author,
            &announcement_link,
            PayloadBuilder::new()
                .masked(&StreamsData::default())?
                .build(),
        )
        .await
        .unwrap();

        tokio::time::delay_for(Duration::from_secs(10)).await;
    }
}

///
/// Send a signed message
///
async fn send_signed_data<'a, T, P>(
    client: &mut T,
    author: &mut Author,
    addrs: &Address,
    payload: P,
) -> anyhow::Result<Address>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
    P: PacketPayload,
{
    let signed_packet_link = {
        let msg = author
            .sign_packet(&addrs, &payload.public_data(), &payload.masked_data())
            .map_err(|_| anyhow::anyhow!("Error to create signed packet"))?;
        println!("\tSigned Message ID={}", msg.link.msgid);
        send_message(client, &msg).await.unwrap();
        msg.link.clone()
    };
    Ok(signed_packet_link)
}
