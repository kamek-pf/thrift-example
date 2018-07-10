## Thrift demo
In this repo you'll see an example of Thrift, a RPC framework we will use to make API calls between our own services.

A RPC framework allows you to use an programming language agnostic IDL to describe what a service can do (see `fake-service.thrift`).
Then it will generate all the boiler plate code you don't actually care about when you write a service (serialization, deserialization, transport layer, ...).

In short, it let's you focus on the feature you need to implement. You just need to pull the definition file, generate the code and implement the functions.

## TypeScript server
TL;DR :
From this :
```thrift
service FakeThingy {
    void ping(),
    i32 add(1: required i32 num1, 2: required i32 num2),
    double divide(1: required double num1, 2: required double num2) throws (1: required InvalidOperation ouch),
    FakeProfile get_profile(1: required i32 id)
}
```

Implement the body of these functions :
```typescript
const myServiceHandler = {
    ping(): void {...},
    add(left: number, right: number): number {...},
    divide(left: number, right: number): number {...},
    async get_profile(): Promise<FakeProfile> {...}
}
```

And you're done. See `ts-server` for a full server example in typescript.

## Client side of things
The client part works the same way : generate code from the definition file, initialize a client and start calling functions.
