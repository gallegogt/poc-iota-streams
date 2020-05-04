use failure::Fallible;
use iota::Client;
use iota_streams::{
    app_channels::api::tangle::{Address, Author, DefaultTW, Message},
    core::tbits::Tbits,
    protobuf3::types::Trytes,
};
use poc::transport::AsyncTransport;
use std::{env, process::exit, str::FromStr};

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
        "\t\tChannel Adress = {} \t <== (^Copy this Address for the Subscriber)\n",
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
        &format!("DATADATADATA"),
    )
    .await
    .unwrap();

    let _link = send_tagged_data(
        &mut api,
        &mut author,
        &keyload_link,
        &format!("DATADATADATADATADATADATADAT"),
    )
    .await
    .unwrap();

    Ok(())
}

async fn send_signed_data<'a, T>(
    client: &mut T,
    author: &mut Author,
    addrs: &Address,
    public_data: &'a str,
) -> Fallible<Address>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
{
    let public: Trytes<DefaultTW> = Trytes(Tbits::from_str(public_data).unwrap());
    let masked: Trytes<DefaultTW> = Trytes(Tbits::from_str("MASKEDPAYLOAD").unwrap());

    let signed_packet_link = {
        let msg = author.sign_packet(&addrs, &public, &masked)?;
        println!("\t\tSigned Message ID={}", msg.link.msgid);
        send_message(client, &msg).await.unwrap();
        msg.link.clone()
    };
    Ok(signed_packet_link)
}

async fn send_tagged_data<'a, T>(
    client: &mut T,
    author: &mut Author,
    addrs: &Address,
    public_data: &'a str,
) -> Fallible<Address>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
{
    let public_payload: Trytes<DefaultTW> = Trytes(Tbits::from_str(public_data).unwrap());
    let masked_payload: Trytes<DefaultTW> = Trytes(Tbits::from_str("MASKEDPAYLOAD").unwrap());

    let tagged_packet_link = {
        let msg = author.tag_packet(&addrs, &public_payload, &masked_payload)?;
        println!("\t\tTag Message Tag={}", msg.link.msgid);
        send_message(client, &msg).await.unwrap();
        msg.link.clone()
    };
    Ok(tagged_packet_link)
}

async fn send_message<T>(transport: &mut T, message: &Message) -> Fallible<()>
where
    T: AsyncTransport + Send,
    <T>::SendOptions: Copy + Default + Send,
{
    let _transaction = transport.send_message(message).await?;
    Ok(())
}
