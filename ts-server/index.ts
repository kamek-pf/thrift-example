import {
    createWebServer,
    TCompactProtocol,
    TBufferedTransport,
} from 'thrift'

import { FakeThingy, FakeProfile, InvalidOperation } from './codegen'

// This handler implements the service,
// that's pretty much all you need to do
const serviceHandler = {
    ping(): void {
        console.log("pong");
    },
    add(left: number, right: number): number {
        return left + right;
    },
    divide(left: number, right: number): number {
        if (right === 0) {
            throw new InvalidOperation({
                code: 500,
                why: "lol noob"
            });
        }

        return left / right;
    },
    // You can implement things synchronously or asynchonously !
    async get_profile(): Promise<FakeProfile> {
        // call APIs, fetch from database ...
        return new FakeProfile({
            id: 15,
            first_name: "Test",
            last_name: "Icule"
        })
    }
}

// The rest is just noise, configuring the transport layer,
// serialization protocol ...

// ServiceOptions: The I/O stack for the service
const myServiceOpts = {
    handler: serviceHandler,
    processor: FakeThingy,
    protocol: TCompactProtocol,
    transport: TBufferedTransport
}

// ServerOptions: Define server features
const serverOpt = {
    services: {
        '/': myServiceOpts
    }
}

// Create and start the web server
const port: number = 8045;
createWebServer(serverOpt).listen(port, () => {
    console.log(`Thrift server listening on port ${port}`)
})
