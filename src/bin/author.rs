use failure::Fallible;
use iota::Client;
use iota_streams::{
    app_channels::{
        api::tangle::{Address, Author, DefaultTW},
        message,
    },
    protobuf3::types::Trytes,
};
use poc::transport::{recv_messages, send_message, AsyncTransport, PayloadBuilder, TPacket};
use std::{env, process::exit, time::Duration};

#[tokio::main]
async fn main() -> Fallible<()> {
    let args: Vec<_> = env::args().collect::<Vec<_>>();
    if args.len() < 2 {
        eprintln!("Usage: author <SEED>");
        exit(1);
    }

    // tangle client
    let mut api = Client::new("https://nodes.comnet.thetangle.org:443");
    // Create the author
    let mut author = Author::new(&args[1], 3, true);

    println!(
        "\t\tChannel Adress = {} <== (^Copy this Address for the Subscriber)\n",
        author.channel_address()
    );

    let (announcement_address, announcement_tag) = {
        let msg = &author.announce()?;
        println!(
            "\t\tAnnouncement Message Tag={} \t <= (^Copy this Tag for the Subscriber)",
            msg.link.msgid
        );

        send_message(&mut api, msg).await.unwrap();

        (msg.link.appinst.to_string(), msg.link.msgid.to_string())
    };

    // Annuncement Link
    let announcement_link = Address::from_str(&announcement_address, &announcement_tag).unwrap();

    // Simulate wait for subscribers
    // =======================================
    println!("Waiting 60s for subscribers ...\n");
    println!("Open the susbscriber NOW...\n");
    tokio::time::delay_for(Duration::from_secs(60)).await;
    // =======================================

    println!("Accept subscribers....");
    accept_suscribers(&mut api, &mut author, &announcement_link).await?;

    println!("Share keyload for everyone:");
    let keyload_link = {
        let msg = author.share_keyload_for_everyone(&announcement_link)?;
        println!("\t\tShare KeyLoad Message Tag={}", msg.link.msgid);
        send_message(&mut api, &msg).await.unwrap();
        msg.link
    };

    let _link_signed = send_signed_data(
        &mut api,
        &mut author,
        &announcement_link,
        PayloadBuilder::new()
            .masked_data("Signed Data from Text")
            .build(),
    )
    .await
    .unwrap();

    let _link = send_tagged_data(
        &mut api,
        &mut author,
        &keyload_link,
        PayloadBuilder::new()
            .masked_data("Tagged Payload as Text")
            .build(),
    )
    .await
    .unwrap();

    Ok(())
}

async fn send_signed_data<'a, T, P>(
    client: &mut T,
    author: &mut Author,
    addrs: &Address,
    payload: P,
) -> Fallible<Address>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
    P: TPacket,
{
    let public: Trytes<DefaultTW> = payload.public_data();
    let masked: Trytes<DefaultTW> = payload.masked_data();

    let signed_packet_link = {
        let msg = author.sign_packet(&addrs, &public, &masked)?;
        println!("\t\tSigned Message ID={}", msg.link.msgid);
        send_message(client, &msg).await.unwrap();
        msg.link.clone()
    };
    Ok(signed_packet_link)
}

async fn send_tagged_data<'a, T, P>(
    client: &mut T,
    author: &mut Author,
    addrs: &Address,
    payload: P,
) -> Fallible<Address>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
    P: TPacket,
{
    let public: Trytes<DefaultTW> = payload.public_data();
    let masked: Trytes<DefaultTW> = payload.masked_data();

    let tagged_packet_link = {
        let msg = author.tag_packet(&addrs, &public, &masked)?;
        println!("\t\tTag Message Tag={}", msg.link.msgid);
        send_message(client, &msg).await.unwrap();
        msg.link.clone()
    };
    Ok(tagged_packet_link)
}

///
/// Accept all Subscribers
///
async fn accept_suscribers<'a>(
    client: &mut Client<'a>,
    author: &mut Author,
    channel_link: &Address,
) -> Fallible<()> {
    let msg_list = recv_messages(client, channel_link).await?;
    msg_list
        .iter()
        .filter_map(|msg| match msg.parse_header() {
            Ok(preparsed) => {
                if preparsed.check_content_type(message::subscribe::TYPE) {
                    Some(preparsed)
                } else {
                    None
                }
            }
            Err(_) => None,
        })
        .for_each(|preparsed| {
            author.unwrap_subscribe(preparsed).unwrap();
            println!("Add new subscriber ....");
        });

    // for msg in msg_list.iter() {
    //     let preparsed = msg.parse_header()?;
    //     if preparsed.check_content_type(message::subscribe::TYPE) {
    //         println!("Message Type {}", preparsed.content_type());
    //         author.unwrap_subscribe(preparsed)?;
    //     }
    // }

    Ok(())
}
