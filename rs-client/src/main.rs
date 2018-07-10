extern crate ordered_float;
extern crate thrift;
extern crate try_from;

mod fake_service;

use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::transport::{TBufferedReadTransport, TBufferedWriteTransport, TIoChannel, TTcpChannel};

use fake_service::{FakeThingySyncClient, TFakeThingySyncClient};

fn main() {
    let addr = "127.0.0.1:8045";
    let mut channel = TTcpChannel::new();

    // setup tcp connections
    channel.open(addr).unwrap();
    let (inc, outc) = channel.split().unwrap();

    // setup protocols
    let input_proto = TCompactInputProtocol::new(TBufferedReadTransport::new(inc));
    let output_proto = TCompactOutputProtocol::new(TBufferedWriteTransport::new(outc));

    // start calling stuff
    let mut c = FakeThingySyncClient::new(input_proto, output_proto);

    let res = c.add(3, 5);
    println!("kek {:?}", res);
}
