# Chaum-Pedersen ZKP Auth Protocol

This project implements the ZKP Chaum-Pedersen protocol in a very simple client-server architecture, using gRPC to define the interfaces between client and server.

## Prerequisites
To build locally, you must have the following installed:

- rust v1.7.5
- protobuf-compiler 
- libprotobuf-dev

## Parameters

The Chaum-Pedersen protocol requires 4 parameters (P, Q, G, H) to be specified for operation. These parameters must be identical between both the Client and Server. This application will attempt to load these parameters from the environment at the following keys:
```env
CP_P
CP_Q
CP_G
CP_H
```

If a `.env` file exists in the execution directory it will automatically be loaded. For convenience, a sample set of initial parameters is provided in the included `.env` file, and the 

### Generating New Parameters
If you would like to generate fresh Chaum-Pedersen parameters, run
```bash
./zkp-auth generate -o .env # optional - output file path
```

## Testing

To run all tests, both integration and unit for the project, execure:
```bash
cargo test
```

## Building
The program binary may be built by running

```bash
make build
```

Output binary can then be found at `target/release/zkp-auth`

## Running
> Note: To see more verbose logging output, set `RUST_LOG=trace` before running

### Server
To run the zkp-auth server:
```bash
./zkp-auth server -l 0.0.0.0:8080 # optional -l specifies listen address
```

### Client
To run a barebones zkp-auth client, which will attempt to register and prove a secret value with the server:
```bash
./zkp-auth client \
  -s 127.0.0.1:8080 \ # optional server address (default: 127.0.0.1:8080) 
  -u username \
  -p 123 
```

#### Expected Output
```bash
Authentication successful. Session ca6e29d9-4234-4387-a692-65c7789a373f # unique session UUID
```

> Note: providing both `-u` and `-p` flags will make the program run non-interactively. If they are ommitted, the user will be prompted to enter them at runtime.

## Running in Docker
For convenience, a `docker-compose` file is included which will build & run both the client and server applications in separate containers. 

This `docker-compose` captures the _Parameters_ from the environment (or, `.env` file by default). So, before building please ensure you either have a `.env` file or parameters exist in the current shell's environment.

To run, execute:

```bash
docker compose up
```


# Design Documentation

For the purposes of this challenge, implementation of this protocol is kept very simple and lightweight.  

- All code relevant to the server is found in `server.rs`, while client code is in `client.rs`. 
- Parameters, parameter generation, and math operations are found in `chaum_pedersen.rs`
- User & auth session state is simply stored in-memory using hashmaps - this is found in `db.rs`.
- `main.rs` exposes a command-line interface for interacting with the client and server.
- `tests/integration_test.rs` runs both client and server, and verifies that the entire proof process and communication works end-to-end


## Potential Improvements
- Backend user session management -  currently, a user can only register once. Allow a method for user to overwrite their y1/y2 params. 
- Generalize `db.rs` into a set of traits, to allow for other databases to be used (i.e, persist to SQLite, instead of just in-memory) 
- Support for [RFC5114 MODP groups with generators](https://www.rfc-editor.org/rfc/rfc5114) - parse Hex values from ENV instead of numbers
- Decoupling/refactoring the Chaum-Pedersen functionality out of `server.rs` , and into a separate library so it could be used with a different front-end (ex, websockets instead of gRPC)
- CLI: Client-side state persistence, allowing multiple proofs to be registered under a single `user` / `y1/y2`
  - This would also require more CLI commands to be added, but exact implementation would depend on final use case requirements