import {
    createClient,
    createConnection,
    TCompactProtocol,
    TBufferedTransport,
} from 'thrift'

import { FakeThingy } from './codegen'

// The location of the server endpoint
const CONFIG = {
    hostName: '0.0.0.0',
    port: 8045
}

const options = {
    transport: TBufferedTransport,
    protocol: TCompactProtocol,
}

const connection = createConnection(CONFIG.hostName, CONFIG.port, options)
const thriftClient: FakeThingy.Client = createClient(FakeThingy.Client, connection)

// All client methods return a Promise of the expected result.
thriftClient.add(10, 7).then((result: number) =>{
    console.log(`result: ${result}`)
})
.catch(() => {
    console.log("NONONONONO");
})
