# Examples of IOTA Streams library

## Examples:

* [E01 Simple Author](examples/e01-author.rs): Publish random data
* [E01 Simple Subscriber](examples/e01-subscriber.rs): Fetch all message published by the Simple Author example
* [E02 Simple Author with Keyload](examples/e02-author-keyload.rs): Publish random data
* [E02 Simple Subscriber with keyload](examples/e02-subscriber-keyload.rs): Fetch all message published by the Simple Author with keyload example

## Outputs Samples

* [E01 Simple Author](examples/e01-author.rs): Publish random data

**Output:**

```bash
➜  iota_stream_poc git:(master) ✗ cargo run -j4 --example e01-author --release -- --seed ABCD9999
   Compiling iota_stream_poc v0.1.0
    Finished release [optimized] target(s) in 1m 33s
     Running `target/release/examples/e01-author --seed ABCD9999`
Channel Address (Copy this Address for the Subscribers):
        2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000

Announcement Message Tag:
        e2feafdd5c6a72cef26ea3b2

DATA=StreamsData { ts: 2021-04-01T18:19:49.269893, desc: "uT8NjU2LtKOWLroFqjbb 67k1", temperature: 791.40643, pressure: 49605.402 }
        Tagged Message ID=d3c13a14edb894950931b88f <2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000:d3c13a14edb894950931b88f>
        Seq=None
DATA=StreamsData { ts: 2021-04-01T18:20:08.598309, desc: "82kqSmRueHZFQZKllQcG2sfQ08VYg8d8P,kyvNLR", temperature: 471.7496, pressure: 57465.645 }
        Tagged Message ID=c836ed68e5f4b350dacd8dcc <2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000:c836ed68e5f4b350dacd8dcc>
        Seq=None
DATA=StreamsData { ts: 2021-04-01T18:20:25.120094, desc: "MxizFWXXbRJ LnDnRljKtDYMVu", temperature: 1101.543, pressure: 44863.094 }
        Tagged Message ID=ecb62ae87bfcf22aa9c87d95 <2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000:ecb62ae87bfcf22aa9c87d95>
        Seq=None
DATA=StreamsData { ts: 2021-04-01T18:20:39.518442, desc: "E1;NoCWEzbPUacWEyN1FRHJQ7IyWTqID", temperature: 1056.0802, pressure: 30025.535 }
        Tagged Message ID=7d4d43d4f44b54396897c48b <2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000:7d4d43d4f44b54396897c48b>
        Seq=None
DATA=StreamsData { ts: 2021-04-01T18:20:52.934573, desc: ".YPKhPdbe01tDBSD4Gk6uY3", temperature: 204.34595, pressure: 57311.027 }
        Tagged Message ID=c6e7eaf8d1a5bbaf4ce49baa <2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000:c6e7eaf8d1a5bbaf4ce49baa>
        Seq=None
DATA=StreamsData { ts: 2021-04-01T18:21:10.341918, desc: "BzF5X7ggp4PJLMDYkQpkqE", temperature: 689.1529, pressure: 33760.57 }
        Tagged Message ID=d34f38881ce7e59c965c7a1a <2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000:d34f38881ce7e59c965c7a1a>
        Seq=None
DATA=StreamsData { ts: 2021-04-01T18:21:23.448566, desc: "z0ZRwe7ph mcYq,BvM9 6g03Q8ku MRBP0", temperature: 738.4544, pressure: 31922.246 }
        Tagged Message ID=32f2281ff44539de7af54178 <2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000:32f2281ff44539de7af54178>
        Seq=None
DATA=StreamsData { ts: 2021-04-01T18:21:35.530034, desc: "SpPBlGyQ,teAc3wEa0nTnJF", temperature: 758.47656, pressure: 19224.47 }
        Tagged Message ID=ad3bf350c51230f00a791d7e <2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000:ad3bf350c51230f00a791d7e>
        Seq=None
```


* [E01 Simple Subscriber](examples/e01-subscriber.rs): Fetch all message published by the Simple Author example

**Output**

```bash
➜  iota_stream_poc git:(master) ✗ cargo run --example e01-subscriber -j 4 --release -- --seed SEED99 --channel 2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000 --announcement-tag e2feafdd5c6a72cef26ea3b2
   Compiling iota_stream_poc v0.1.0
    Finished release [optimized] target(s) in 1m 30s
     Running `target/release/examples/e01-subscriber --seed SEED99 --channel 2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000 --announcement-tag e2feafdd5c6a72cef26ea3b2`
Channel Address=2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000
Announcement Tag ID=e2feafdd5c6a72cef26ea3b2

Subscriber Channel Address 2bd36986053a06f546e0e3f525ad849e5cc1610fe65ade1cc4e4312bac5242810000000000000000

 0.- Tagged Public Packet:
        StreamsData { ts: 2021-04-01T18:05:06.878578, desc: " thBMd3aC8E0ww;Vz.yzpUyYeHW0aEFln", temperature: 1107.0509, pressure: 58425.793 }

 1.- Tagged Public Packet:
        StreamsData { ts: 2021-04-01T18:05:20.363173, desc: "u3OAZpcgQ13Hr63Q.ALxVGkQ", temperature: 43.321632, pressure: 120806.27 }

 2.- Tagged Public Packet:
        StreamsData { ts: 2021-04-01T18:05:34.189191, desc: "1V66v4AYxZzEpsqcV5U9uJHAaBmWVdvlYFZhxgFaV;LzkGs", temperature: 366.8332, pressure: 77214.63 }

 3.- Tagged Public Packet:
        StreamsData { ts: 2021-04-01T18:05:52.158391, desc: "UVbrvOQbia8oOubatQjjYSVhwvvBfQ,d10.byast", temperature: 1060.3746, pressure: 75965.875 }

 4.- Tagged Public Packet:
        StreamsData { ts: 2021-04-01T18:06:03.912861, desc: "a6eStEEOF2BlQ,c.DbF2T23EYMwnVF34;Fxy1vEES4;ZBHUn2", temperature: 978.01184, pressure: 8243.126 }

 5.- Tagged Public Packet:
        StreamsData { ts: 2021-04-01T18:06:15.661745, desc: "4V,NsNsk2aYmmDXMi,k,SY", temperature: 445.85992, pressure: 116675.04 }

 6.- Tagged Public Packet:
        StreamsData { ts: 2021-04-01T18:06:37.271351, desc: "I3D7O8o ETJr", temperature: 915.38324, pressure: 81221.91 }

 7.- Tagged Public Packet:
        StreamsData { ts: 2021-04-01T18:06:50.860827, desc: "Gw 1F3nPZdyNqz1bJbQN4LaIGLk eIleZ", temperature: 803.75226, pressure: 49793.586 }

No more messages in sequence.

```
