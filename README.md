# PoC of IOTA STREAMS

It just Proof of Concept

### Simple behavior of Author and Subscribe

* First exec the author

```bash
cargo run --release --bin author <YOUR SEED>
```

* When the author finished exec the subscriber

```bash
cargo run --release --bin subscriber <CHANNEL ID> <ANNOUNCEMENT TAG>
```


#### Outputs Samples:

Author:

```
                Channel Adress = E9DTMWVVASPBBSALIPINBMKIVPLSMQINYNPYDPIRRYQTWGYVNHVLGOLSCZQNYZFUKYVOXNRVZFA9NJDAF       <== (^Copy this Address for the Subscriber)

                Announcement Message Tag=CVQQZ9ZKNJSUMEROKFGREN9ZXMX     <= (^Copy this Tag for the Subscriber)
Share keyload for everyone:
                Share KeyLoad Message Tag=EURSRPKTMGRVVURUGJ9BHNLAJBI
                Signed Message ID=TDPJJUHONEXPQJDKMEKQPXRDGAA
                Tag Message Tag=ETICUKGGLPEHLWLSLXUQLZWPTAN
```

Subscriber:

```
Channel Address=E9DTMWVVASPBBSALIPINBMKIVPLSMQINYNPYDPIRRYQTWGYVNHVLGOLSCZQNYZFUKYVOXNRVZFA9NJDAF
Tag ID=CVQQZ9ZKNJSUMEROKFGREN9ZXMX
Subscribe to the channel:
                Message ID=XEESBCSXARJQD9KCZDOSLUQRXTS

Receive 11 Messages
Public Packet: DATADATADATA
Masked Packet: MASKEDPAYLOAD
Public Packet: DATADATADATA
Masked Packet: MASKEDPAYLOAD
Public Packet: DATADATADATA
Masked Packet: MASKEDPAYLOAD
```

