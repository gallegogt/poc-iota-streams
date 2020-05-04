use failure::{ensure, Fallible};
use iota_streams::app_channels::{
    api::tangle::{Address, Message, Subscriber},
    message,
};
use poc::transport::AsyncTransport;
use std::env;

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

    let msg_list = recv_messages(&mut client, &subscribe_link).await?;
    println!("\nReceive {} Messages", msg_list.len());

    for msg in msg_list.iter() {
        let preparsed = msg.parse_header()?;
        if preparsed.check_content_type(message::signed_packet::TYPE) {
            match subscriber.unwrap_signed_packet(preparsed.clone()) {
                Ok((unwrapped_public, unwrapped_masked)) => {
                    println!("Public Packet: {}", unwrapped_public.to_string());
                    println!("Masked Packet: {}", unwrapped_masked.to_string());
                }
                Err(e) => println!("Signed Packet Error: {}", e),
            }
            continue;
        }
        if preparsed.check_content_type(message::tagged_packet::TYPE) {
            let rs = subscriber.unwrap_tagged_packet(preparsed.clone());
            ensure!(rs.is_err());
            match rs {
                Ok((unwrapped_public, unwrapped_masked)) => {
                    println!("Tagged Public Packet: {}", unwrapped_public.to_string());
                    println!("Tagged Masked Packet: {}", unwrapped_masked.to_string());
                }
                Err(e) => println!("Tagged Packet Error: {}", e),
            }
            continue;
        }
        if preparsed.check_content_type(message::keyload::TYPE) {
            let rs = subscriber.unwrap_keyload(preparsed.clone());
            ensure!(rs.is_err());
            continue;
        }
    }

    Ok(())
}

async fn recv_messages<T>(transport: &mut T, addr: &Address) -> Fallible<Vec<Message>>
where
    T: AsyncTransport,
    <T>::RecvOptions: Copy + Default + Send,
{
    transport
        .recv_messages_with_options(addr, T::RecvOptions::default())
        .await
}

async fn recv_message<T>(transport: &mut T, addr: &Address) -> Fallible<Option<Message>>
where
    T: AsyncTransport + Send,
    <T>::RecvOptions: Copy + Default + Send,
{
    transport.recv_message(addr).await
}

async fn send_message<T>(transport: &mut T, message: &Message) -> Fallible<()>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
{
    transport.send_message(message).await?;
    Ok(())
}

