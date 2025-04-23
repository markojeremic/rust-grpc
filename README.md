# Rust gRPC Example with tonic

A simple Rust project demonstrating how to build a gRPC server and client using [`tonic`](https://crates.io/crates/tonic) and [`prost`](https://crates.io/crates/prost). This example shows how a client can request the model number of a model by ID, and the server responds with the respective model number.
Main point of this exercise is to learn how to define and implement gRPC services using [`tonic`] crates, as well as to get in touch with multi-crate project structure (client-server).

## Prerequisites

- Rust (1.86.0 or newer)
- Protobuf compiler (protoc)

## How It Works

- The `.proto` file defines a `ModelService` with one RPC: `GetModelNumber`.
- `tonic_build` compiles this into Rust types.
- The server implements the logic for `GetModelNumber` (returns a model number based on model ID).
- The client sends a request to the server and prints the model number response.

##  Running Locally

### 1. Clone the repo
```git clone https://github.com/your-username/grpc-example```

### 2. Build the project
```cargo build```

### 3. Start the server from Terminal
```cargo run -p server```

You should see the following log:

```Server listening on [::1]:50051```

### 4. Start the client from Terminal

```cargo run -p client```

You should see the following log:

```RESPONSE: ModelResponse { model_num: 4200 }```




