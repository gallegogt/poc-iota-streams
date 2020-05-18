//!
//! Author Of Linked Messages
//!
//! How run this example:
//!
//! ```bash
//!   cargo run --example author-linked-messages --release -- --seed <SEED> [--mss_height 3]
//! ```
//!
use clap::{App, Arg};
use iota_streams::app_channels::api::tangle::{Address, Author};
use poc::{
    sample::StreamsData,
    transport::{
        payload::{PacketPayload, PayloadBuilder},
        send_message, AsyncTransport, IotaTransport,
    },
};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new("Author Of Linked Messages")
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
                .help("The Tangle Url, Default: https://nodes.comnet.thetangle.org:443"),
        )
        .arg(
            Arg::with_name("mss_height")
                .short("m")
                .long("mss_height")
                .takes_value(true)
                .help("Merkle Tree Signature Height, Default: 3"),
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

    // Initialize The IOTA Client
    //
    let mut client = IotaTransport::add_node(api_url).unwrap();

    // Create the author
    //
    let mut author = Author::new(seed, mss_height, true);

    println!("\rChannel Address (Copy this Address for the Subscribers):");
    println!("\t{}\n", author.channel_address());

    // Creare Announcement Tag
    //
    let mut link_addr = {
        let msg = &author
            .announce()
            .map_err(|_| anyhow::anyhow!("Error creating announce message"))?;

        println!("Announcement Message Tag:");
        println!("\t{}\n", msg.link.msgid);

        send_message(&mut client, msg).await.unwrap();

        msg.link.clone()
    };

    let shk_message = author
        .share_keyload_for_everyone(&link_addr)
        .map_err(|_| anyhow::anyhow!("Error creating share keyload for everyone message"))?;

    print!(
        "Share Keyload For Everyone Message Tag={} ..",
        shk_message.link.msgid
    );
    send_message(&mut client, &shk_message).await.unwrap();
    print!("[OK]\n\r");

    // Posible numbers of messages to sign before change the key
    //
    let remaining_sk = 2_u32.pow(mss_height as u32);
    let mut remaining_signed_messages = remaining_sk;

    loop {
        if remaining_signed_messages <= 0 {
            let chk_message = author
                .change_key(&link_addr)
                .map_err(|_| anyhow::anyhow!("Error changing the key"))?;

            // send change key message
            //
            send_message(&mut client, &chk_message).await.unwrap();
            link_addr = chk_message.link.clone();
            remaining_signed_messages = remaining_sk;

            println!("Change Key Message Tag:");
            println!("\t{}\n", chk_message.link.msgid);
            println!("Change Key Message Address:");
            println!("\t{}\n", chk_message.link.appinst.to_string());
        } else {
            remaining_signed_messages = remaining_signed_messages - 1;
        }

        link_addr = send_signed_data(
            &mut client,
            &mut author,
            &link_addr,
            PayloadBuilder::new()
                .masked(&StreamsData::default())
                .build(),
        )
        .await
        .unwrap();

        tokio::time::delay_for(Duration::from_secs(1)).await;
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

        print!("\tSending Signed Message ID={} ...", msg.link.msgid);
        match send_message(client, &msg).await {
            Ok(_) => {
                print!("[OK] \n\r");
            }
            Err(e) => {
                print!("[ERROR] {:#?}\n\r", e);
            }
        }

        msg.link.clone()
    };
    Ok(signed_packet_link)
}
