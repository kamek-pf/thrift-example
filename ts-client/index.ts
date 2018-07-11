import {
    createClient,
    createConnection,
    TBinaryProtocol,
    TBufferedTransport,
} from "thrift";

import { FakeThingy } from "./codegen";

// The location of the server endpoint
const hostName = "0.0.0.0";
const port = 8045;

const options = {
    transport: TBufferedTransport,
    protocol: TBinaryProtocol,
};

const connection = createConnection(hostName, port, options);
const thriftClient = createClient(FakeThingy.Client, connection);

// All client methods return a Promise of the expected result.
thriftClient
    .add(10, 6)
    .then(result => {
        console.log(`result add : ${result}`);
    })
    .catch(() => {
        console.log("NONONONONO");
    });

thriftClient
    .divide(30, 3)
    .then(result => {
        console.log(`result divide: ${result}`);
    })
    .catch(() => {
        console.log("NONONONONO");
    });
