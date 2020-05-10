use failure::{ensure, Fallible};
use iota_conversion::trytes_converter::to_string as trytes_to_string;
use iota_streams::app_channels::{
    api::tangle::{Address, Subscriber},
    message,
};
use poc::transport::{recv_message, recv_messages, send_message};
use std::{env, time::Duration};

#[tokio::main]
async fn main() -> Fallible<()> {
    let args: Vec<_> = env::args().collect::<Vec<_>>();

    // tangle client
    let mut client = iota::Client::new("https://nodes.comnet.thetangle.org");
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
        let preparsed = msg.parse_header()?;
        if preparsed.check_content_type(message::announce::TYPE) {
            subscriber.unwrap_announcement(preparsed)?;
        }
    }

    let subscribe_link = {
        println!("Subscribe to the channel:");
        let msg = subscriber.subscribe(&announcement_link)?;
        println!("\t\tMessage ID={}", msg.link.msgid);
        send_message(&mut client, &msg).await?;
        msg.link.clone()
    };

    // Simulate waiting for Actor access
    println!("Waiting 60s for Author send the keyload message....");
    tokio::time::delay_for(Duration::from_secs(60)).await;

    let msg_list = recv_messages(&mut client, &subscribe_link).await?;
    println!("\nReceive {} Messages", msg_list.len());

    for (_idx, msg) in msg_list.iter().enumerate() {
        let preparsed = msg.parse_header()?;
        if preparsed.check_content_type(message::signed_packet::TYPE) {
            match subscriber.unwrap_signed_packet(preparsed.clone()) {
                Ok((unwrapped_public, unwrapped_masked)) => {
                    println!(
                        "Public Packet: {}",
                        trytes_to_string(&unwrapped_public.to_string())?
                    );
                    println!(
                        "Signed Masked Packet: {}",
                        trytes_to_string(&unwrapped_masked.to_string())?
                    );
                }
                Err(e) => println!("Signed Packet Error: {}", e),
            }
            continue;
        }

        if preparsed.check_content_type(message::tagged_packet::TYPE) {
            let rs = subscriber.unwrap_tagged_packet(preparsed.clone());
            ensure!(rs.is_ok());
            match rs {
                Ok((unwrapped_public, unwrapped_masked)) => {
                    println!(
                        "Tagged Public Packet: {}",
                        trytes_to_string(&unwrapped_public.to_string())?
                    );
                    println!(
                        "Tagged Masked Packet: {}",
                        trytes_to_string(&unwrapped_masked.to_string())?
                    );
                }
                Err(e) => println!("Tagged Packet Error: {}", e),
            }
            continue;
        }

        if preparsed.check_content_type(message::keyload::TYPE) {
            let rs = subscriber.unwrap_keyload(preparsed.clone());
            ensure!(rs.is_ok());
            continue;
        }
    }

    Ok(())
}
