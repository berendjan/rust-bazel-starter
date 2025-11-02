# rust-bazel-starter

This is `rust` + `bazel` + `grpc` template repository for creating backend applications.

The current implementation uses [protoc and protoc-gen-rust-grpc](https://github.com/hyperium/tonic/tree/master/protoc-gen-rust-grpc) which can take a long time to build.

Run the following to start the grpc server locally.
```bash
bazel run //rust/grpc_server
```

## docs

See `docs/rules_rust.md` for helpful development information
