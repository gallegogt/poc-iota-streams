use iota_streams::{
    app_channels::{
        api::tangle::{Address, DefaultTW, Subscriber},
        message,
    },
    protobuf3::types::Trytes,
};
use poc::{
    sample::StreamsData,
    transport::{payload::Payload, recv_message, recv_messages, send_message, IotaTransport},
};
use std::{env, time::Duration};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<_> = env::args().collect::<Vec<_>>();

    // tangle client
    let mut client = IotaTransport::add_node("https://nodes.comnet.thetangle.org:443").unwrap();
    let mut subscriber = Subscriber::new(
        "CWRRJXOOOHTEE9LQ99MEMGMYIDHYJEIWODFUJTXH9UNRRHHOZOOFUBZVXPWTXPLGSIQNSDZRQFEZEAXZD",
        true,
    );

    let announcement_address = &args[1];
    let announcement_tag = &args[2];

    println!("Channel Address={}", announcement_address);
    println!("Tag ID={}", announcement_tag);

    // Annuncement Link
    let announcement_link = Address::from_str(&announcement_address, &announcement_tag).unwrap();
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

    let subscribe_link = {
        println!("Subscribe to the channel:");
        let msg = subscriber
            .subscribe(&announcement_link)
            .map_err(|_| anyhow::anyhow!("Error on subscriber link"))?;

        println!("\t\tMessage ID={}", msg.link.msgid);
        send_message(&mut client, &msg).await?;
        msg.link.clone()
    };

    // Simulate waiting for Actor access
    println!("Waiting 65s for Author send the keyload message....");
    tokio::time::delay_for(Duration::from_secs(65)).await;

    let msg_list = recv_messages(&mut client, &subscribe_link).await?;
    println!("\nReceive {} Messages", msg_list.len());

    for (_idx, msg) in msg_list.iter().enumerate() {
        let preparsed = msg
            .parse_header()
            .map_err(|_| anyhow::anyhow!("Error parsing the message header"))?;
        if preparsed.check_content_type(message::signed_packet::TYPE) {
            match subscriber.unwrap_signed_packet(preparsed.clone()) {
                Ok((unwrapped_public, unwrapped_masked)) => {
                    print_received_data("Signed", unwrapped_public, unwrapped_masked);
                }
                Err(e) => println!("Signed Packet Error: {}", e),
            }
            continue;
        }

        if preparsed.check_content_type(message::tagged_packet::TYPE) {
            let rs = subscriber.unwrap_tagged_packet(preparsed.clone());
            match rs {
                Ok((unwrapped_public, unwrapped_masked)) => {
                    print_received_data("Tagged", unwrapped_public, unwrapped_masked);
                }
                Err(e) => println!("Tagged Packet Error: {}", e),
            }
            continue;
        }

        if preparsed.check_content_type(message::keyload::TYPE) {
            let rs = subscriber.unwrap_keyload(preparsed.clone());
            if rs.is_err() {
                println!("Error on unwrap_keyload")
            }
            continue;
        }
    }

    Ok(())
}

///
/// Print Sended Data
///
fn print_received_data<T>(prefix: T, public: Trytes<DefaultTW>, masked: Trytes<DefaultTW>)
where
    T: Into<String>,
{
    let p_data: Option<StreamsData> = Payload::unwrap_data(&public).unwrap();
    let m_data: Option<StreamsData> = Payload::unwrap_data(&masked).unwrap();
    let pfx = prefix.into();

    match p_data {
        Some(d) => println!("\n {} Public Packet: \n \t{:?}\n", pfx, d),
        None => {}
    }
    match m_data {
        Some(d) => println!("\n {} Masked Packet: \n \t{:?}\n", pfx, d),
        None => {}
    }
}
