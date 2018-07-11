extern crate ordered_float;
extern crate thrift;
extern crate try_from;

mod fake_service;

use ordered_float::OrderedFloat;
use thrift::protocol::{TBinaryInputProtocol, TBinaryOutputProtocol};
use thrift::transport::{TIoChannel, TTcpChannel};

use fake_service::{FakeThingySyncClient, TFakeThingySyncClient};

fn main() {
    let mut client = get_client();

    let res = client.add(10, 5);
    assert_eq!(res.unwrap(), 15);

    let left: OrderedFloat<f64> = 30f64.into();
    let right: OrderedFloat<f64> = 3f64.into();
    let res = client.divide(left, right);
    assert_eq!(res.unwrap(), 10f64.into());

    let res = client.get_profile(15).unwrap();
    assert_eq!(res.id, 15);
    assert_eq!(res.first_name, Some("Test".into()));
}

fn get_client() -> impl TFakeThingySyncClient {
    // setup tcp connection
    let addr = "127.0.0.1:8085";
    let mut channel = TTcpChannel::new();
    channel.open(addr).unwrap();
    let (inc, outc) = channel.split().unwrap();

    // setup protocol
    let input_proto = TBinaryInputProtocol::new(inc, true);
    let output_proto = TBinaryOutputProtocol::new(outc, true);

    // initialize client
    FakeThingySyncClient::new(input_proto, output_proto)
}
