## Code generation

To generate code for TS and node, you need to add the following binary :
```
yarn global add @creditkarma/thrift-typescript
```

Then you can run
```
thrift-typescript ../fake-service.thrift
```

The generated code goes under the `code-gen` folder. It generate everything including interfaces.
You can take a look at the `index.ts`, you don't need to touch the generated code at all.
