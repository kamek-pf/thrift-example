import { createServer, createWebServer, TBinaryProtocol, TBufferedTransport } from "thrift";
import { FakeThingy, FakeProfile, InvalidOperation } from "./codegen";

const httpPort: number = 8080;
const tcpPort: number = 8085;
const hostName: string = "0.0.0.0";

// This handler implements the service,
// that's pretty much all you need to do
const serviceHandler = {
    ping(): void {
        console.log("pong");
    },
    add(left: number, right: number) {
        return left + right;
    },
    divide(left: number, right: number): number {
        if (right === 0) {
            throw new InvalidOperation({
                code: 500,
                why: "you can't do that :(",
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
            last_name: "Icule",
        });
    },
};

// The rest is boilerplate, configuring the transport layer,
// serialization protocol, and running the server
const serviceOptions = {
    handler: serviceHandler,
    processor: FakeThingy.Processor,
    protocol: TBinaryProtocol,
    transport: TBufferedTransport,
};

let server = createServer(FakeThingy, serviceHandler, serviceOptions);
server.listen(tcpPort, hostName);
console.log(`Thrift TCP server listening on port ${tcpPort}`);

const httpServerOptions = {
    services: {
        '/': serviceOptions
    }
}

createWebServer(httpServerOptions).listen(httpPort, () => {
    console.log(`Thrift HTTP server listening on port ${httpPort}`)
});

