import {
    createHttpClient,
    createHttpConnection,
    TBinaryProtocol,
    TBufferedTransport,
} from "thrift";

import { FakeThingy } from "./codegen";

// The location of the server endpoint
const hostName = "0.0.0.0";
const port = 8080;

const options = {
    transport: TBufferedTransport,
    protocol: TBinaryProtocol,
};

const connection = createHttpConnection(hostName, port, options);
const thriftClient = createHttpClient(FakeThingy.Client, connection);

// All client methods return a Promise of the expected result.
const sendCalls = async () => {
    const res = await thriftClient.add(10,6);
    console.log(res);

    const res2 = await thriftClient.divide(30,3);
    console.log(res2);
}

function sleep(ms: number){
    return new Promise(resolve=>{
        setTimeout(resolve,ms)
    })
}

const loop = async () => {
    for (var i = 0; i < 500; ++i) {
        await sendCalls();
        await sleep(5000);
    }
}

loop();
