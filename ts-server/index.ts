import { createServer, TCompactProtocol, TBufferedTransport } from "thrift";

import { FakeThingy, FakeProfile, InvalidOperation } from "./codegen";

// Create and start the web server
const port: number = 8045;
const hostName: string = "0.0.0.0";

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
                why: "you can't do that :(",
            });
        }

        const res = left / right;
        console.log(`${left} / ${right} = ${res}`);

        return res;
    },
    // You can implement things synchronously or asynchonously !
    async get_profile(): Promise<FakeProfile> {
        // call APIs, fetch from database ...
        return new FakeProfile({
            id: 15,
            first_name: "Test",
            last_name: "Icule",
        });
    },
};

// The rest is just noise, configuring the transport layer,
// serialization protocol ...

// ServiceOptions: The I/O stack for the service
const serviceOptions = {
    handler: serviceHandler,
    processor: FakeThingy.Processor,
    protocol: TCompactProtocol,
    transport: TBufferedTransport,
};

let server = createServer(FakeThingy, serviceHandler, serviceOptions);
server.listen(port, hostName);
