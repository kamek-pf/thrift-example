extern crate ordered_float;
extern crate thrift;
extern crate try_from;

mod fake_service;

use ordered_float::OrderedFloat;
use thrift::protocol::{TCompactInputProtocol, TCompactOutputProtocol};
use thrift::transport::{TIoChannel, TTcpChannel};

use fake_service::{FakeThingySyncClient, TFakeThingySyncClient};

fn main() {
    // setup tcp connection
    let addr = "127.0.0.1:8045";
    let mut channel = TTcpChannel::new();
    channel.open(addr).unwrap();
    let (inc, outc) = channel.split().unwrap();

    // setup protocol
    let input_proto = TCompactInputProtocol::new(inc);
    let output_proto = TCompactOutputProtocol::new(outc);

    // start calling stuff
    let mut c = FakeThingySyncClient::new(input_proto, output_proto);

    let res = c.add(10, 5);
    assert_eq!(res.unwrap(), 15);

    let left: OrderedFloat<f64> = 30.0.into();
    let right: OrderedFloat<f64> = 3.0.into();
    let res = c.divide(left, right);
    println!("divide : {:?}", res);

    let res = c.get_profile(15).unwrap();
    println!("profile {:?}", res);
}
