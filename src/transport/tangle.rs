//!
//! Tangle Transport Impl
//!
use async_trait::async_trait;
use chrono::Utc;
use failure::Fallible;
use iota::{
    bundle::{
        Address, Bundle, Hash, Index, Nonce, OutgoingBundleBuilder, Payload, Tag, Timestamp,
        Transaction, TransactionBuilder, TransactionField, Value,
    },
    ternary::TryteBuf,
};
use iota_conversion::Trinary;
use std::{str::FromStr, string::ToString};

use iota_streams::{
    app::{
        message::TbinaryMessage,
        transport::tangle::{AppInst, MsgId, TangleAddress},
    },
    core::tbits::{
        word::{BasicTbitWord, StringTbitWord},
        TbitSlice, Tbits,
    },
};

use crate::transport::base::*;

fn make_empty_tx() -> TransactionBuilder {
    TransactionBuilder::new()
        .with_value(Value::from_inner_unchecked(0))
        .with_obsolete_tag(Tag::zeros())
        .with_index(Index::from_inner_unchecked(0))
        .with_last_index(Index::from_inner_unchecked(0))
        .with_attachment_ts(Timestamp::from_inner_unchecked(0))
        .with_bundle(Hash::zeros())
        .with_trunk(Hash::zeros())
        .with_branch(Hash::zeros())
        .with_attachment_lbts(Timestamp::from_inner_unchecked(0))
        .with_attachment_ubts(Timestamp::from_inner_unchecked(0))
        .with_nonce(Nonce::zeros())
}

fn make_tx<TW>(
    address: &Tbits<TW>,
    tag: &Tbits<TW>,
    msg: &Tbits<TW>,
    timestamp: i64,
) -> TransactionBuilder
where
    TW: StringTbitWord,
{
    debug_assert_eq!(243, address.size());
    debug_assert_eq!(81, tag.size());
    debug_assert_eq!(6561, msg.size());

    make_empty_tx()
        .with_address(Address::from_inner_unchecked(
            TryteBuf::try_from_str(&address.to_string())
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .with_tag(Tag::from_inner_unchecked(
            TryteBuf::try_from_str(&tag.to_string())
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .with_payload(Payload::from_inner_unchecked(
            TryteBuf::try_from_str(&msg.to_string())
                .unwrap()
                .as_trits()
                .encode(),
        ))
        .with_timestamp(Timestamp::from_inner_unchecked(timestamp as u64))
}

fn pad_trits<TW>(n: usize, s: TbitSlice<TW>) -> Tbits<TW>
where
    TW: BasicTbitWord,
{
    if n > s.size() {
        let mut t = Tbits::<TW>::zero(n);
        s.copy_min(&t.slice_mut());
        t
    } else {
        Tbits::<TW>::from_slice(s)
    }
}

fn make_txs<TW>(
    address: &Tbits<TW>,
    tag: &Tbits<TW>,
    msg: &Tbits<TW>,
    timestamp: i64,
) -> OutgoingBundleBuilder
where
    TW: StringTbitWord,
{
    debug_assert_eq!(243, address.size());
    debug_assert_eq!(81, tag.size());
    let msg_part_size = 6561;

    let mut bundle = OutgoingBundleBuilder::new();
    let mut m = msg.slice();
    while !m.is_empty() {
        let trx = make_tx(
            address,
            tag,
            &pad_trits(msg_part_size, m.take_min(msg_part_size)),
            timestamp,
        );
        bundle.push(trx);
        m = m.drop_min(msg_part_size);
    }
    bundle
}

/// As Streams Message are packed into a bundle, and different bundles can have the same hash
/// (as bundle hash is calcualted over some essense fields including `address`, `timestamp`
/// and not including `tag`, so different Messages may end up in bundles with the same hash.
/// This leads that it may not be possible to STREAMS Messages from bundle hash only.
/// So this function also takes into account `address` and `tag` fields.
/// As STREAMS Messages can have the same message id (ie. `tag`) it is advised that STREAMS Message
/// bundles have distinct nonces and/or timestamps.
pub fn msg_to_bundle<TW, F>(
    msg: &TbinaryMessage<TW, F, TangleAddress<TW>>,
    timestamp: i64,
) -> Bundle
where
    TW: StringTbitWord,
{
    let bundle = make_txs(
        msg.link().appinst.tbits(),
        msg.link().msgid.tbits(),
        &msg.body,
        timestamp,
    );
    bundle
        .seal()
        .unwrap()
        .attach_local(Hash::zeros(), Hash::zeros())
        .unwrap()
        .build()
        .unwrap()
}

/// Stripped version of `iota::options::SendTrytesOptions<'a>` due to lifetime parameter.
#[derive(Clone, Copy)]
pub struct SendTrytesOptions {
    pub depth: u8,
    pub min_weight_magnitude: u8,
    pub local_pow: bool,
}

impl Default for SendTrytesOptions {
    fn default() -> Self {
        Self {
            depth: 3,
            min_weight_magnitude: 10,
            local_pow: true,
        }
    }
}

#[async_trait]
impl<'a, TW, F> AsyncTransport<TW, F, TangleAddress<TW>> for iota::Client<'a>
where
    TW: StringTbitWord + Send + Sync + 'static,
    F: Send + Sync + 'static,
{
    type SendOptions = SendTrytesOptions;
    type SendOutput = Vec<Transaction>;
    type RecvOptions = ();

    /// Send a Streams message over the Tangle with the current timestamp and default SendTrytesOptions.
    async fn send_message_with_options(
        &mut self,
        msg: &TbinaryMessage<TW, F, TangleAddress<TW>>,
        opt: Self::SendOptions,
    ) -> Fallible<Self::SendOutput> {
        let timestamp = Utc::now().timestamp();
        let bundle = msg_to_bundle(&msg, timestamp);
        let mut trytes: Vec<Transaction> = bundle.into_iter().map(|x| x).collect();
        trytes.reverse();

        let txs = self
            .send_trytes()
            .min_weight_magnitude(opt.min_weight_magnitude)
            .depth(opt.depth)
            .trytes(trytes)
            .send()
            .await
            .unwrap();
        Ok(txs)
    }

    /// Receive a message.
    /// Receive messages with explicit options.
    async fn recv_messages_with_options(
        &mut self,
        link: &TangleAddress<TW>,
        _opt: Self::RecvOptions,
    ) -> Fallible<Vec<TbinaryMessage<TW, F, TangleAddress<TW>>>> {
        let addr_str = link.appinst.to_string();
        let _tag_str = link.msgid.to_string();

        let hashes_resp = self
            .find_transactions()
            .addresses(&vec![Address::from_inner_unchecked(
                TryteBuf::try_from_str(&addr_str)
                    .unwrap()
                    .as_trits()
                    .encode(),
            )])
            // .tags(&vec![Tag::from_inner_unchecked(
            //     TryteBuf::try_from_str(&tag_str)
            //         .unwrap()
            //         .as_trits()
            //         .encode(),
            // )])
            .send()
            .await
            .unwrap();

        let txs_resp = self
            .get_trytes()
            .hashes(&hashes_resp.hashes)
            .send()
            .await
            .unwrap();
        let txs = bundles_from_transactions(&txs_resp.trytes);
        Ok(txs.iter().map(|bundle| bundle_to_message(bundle)).collect())
    }

    /// Receive a message.
    /// Receive messages with explicit options.
    async fn recv_message_with_options(
        &mut self,
        link: &TangleAddress<TW>,
        _opt: Self::RecvOptions,
    ) -> Fallible<Option<TbinaryMessage<TW, F, TangleAddress<TW>>>> {
        let tag_str = link.msgid.to_string();

        let hashes_resp = self
            .find_transactions()
            .tags(&vec![Tag::from_inner_unchecked(
                TryteBuf::try_from_str(&tag_str)
                    .unwrap()
                    .as_trits()
                    .encode(),
            )])
            .send()
            .await
            .unwrap();

        let txs_resp = self
            .get_trytes()
            .hashes(&hashes_resp.hashes)
            .send()
            .await
            .unwrap();
        let txs = bundles_from_transactions(&txs_resp.trytes);
        let mut msgs: Vec<TbinaryMessage<TW, F, TangleAddress<TW>>> =
            txs.iter().map(|bundle| bundle_to_message(bundle)).collect();
        Ok(msgs.pop())
    }
}

fn bundles_from_transactions(hashes: &Vec<Transaction>) -> Vec<Vec<Transaction>> {
    let mut txs = hashes.clone();
    let mut bundles = Vec::new();

    txs.sort_by(|x, y| {
        x.address()
            .to_inner()
            .as_i8_slice()
            .cmp(&y.address().to_inner().as_i8_slice())
            .then(
                x.tag()
                    .to_inner()
                    .as_i8_slice()
                    .cmp(&y.tag().to_inner().as_i8_slice()),
            )
            // different messages may have the same bundle hash!
            .then(
                x.bundle()
                    .to_inner()
                    .as_i8_slice()
                    .cmp(&y.bundle().to_inner().as_i8_slice()),
            )
            // reverse order of txs will be extracted from back with `pop`
            .then(x.index().to_inner().cmp(&y.index().to_inner()).reverse())
    });

    if let Some(tx) = txs.pop() {
        let mut bundle = vec![tx];
        loop {
            if let Some(tx) = txs.pop() {
                if bundle[0].address() == tx.address()
                    && bundle[0].tag() == tx.tag()
                    && bundle[0].bundle() == tx.bundle()
                {
                    bundle.push(tx);
                } else {
                    bundles.push(bundle);
                    bundle = vec![tx];
                }
            } else {
                bundles.push(bundle);
                break;
            }
        }
    }

    // TODO: Check the bundles
    bundles
    // .into_iter()
    // .filter_map(|txs| {
    //     let mut bundle = IncomingBundleBuilder::new();
    //     txs.into_iter().for_each(|t| bundle.push(t));

    //     match bundle.validate() {
    //         Ok(builder) => Some(builder.build()),
    //         Err(err) => {
    //             println!("Err: {:?}", err);
    //             None
    //         }
    //     }
    // })
    // .collect()
}

fn bundle_to_message<TW, F>(bundle: &Vec<Transaction>) -> TbinaryMessage<TW, F, TangleAddress<TW>>
where
    TW: StringTbitWord,
{
    let tx = &bundle[0];
    let addr: String = tx.address().to_inner().as_i8_slice().trytes().unwrap();
    let tag: String = tx.tag().to_inner().as_i8_slice().trytes().unwrap();

    let appinst = AppInst::from_str(&addr).unwrap();
    let msgid = MsgId::from_str(&tag).unwrap();

    let mut body = Tbits::<TW>::zero(0);

    for tx in bundle.into_iter() {
        body += &Tbits::<TW>::from_str(&tx.payload().to_inner().as_i8_slice().trytes().unwrap())
            .unwrap();
    }
    TbinaryMessage::new(TangleAddress::<TW> { appinst, msgid }, body)
}
