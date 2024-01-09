# Chaum-Pedersen ZKP Auth Protocol

This project implements the ZKP Chaum-Pedersen protocol in a very simple client-server architecture, using gRPC to define the interfaces between client and server.

## Prerequisites
To build locally, you must have the following installed:

- Rust v1.7.5
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

> Note: providing both `-u` and `-p` flags will make the program run non-interactively. If they are ommitted, the user will be prompted to enter them at runtime.

## Improvements
- Split up client/server into completely different packages for separate deployment (together now for ease of use / simplicity)
- Backend user session tracking - can only register once. Maybe allow a method for user to overwrite their y1/y2 params
- Module for proper database (i.e, persist to SQLite for instance, instead of just in-memory) 

More Information 
