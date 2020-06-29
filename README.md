# naia-socket

A cross-platform (including Wasm) Socket API that wraps unreliable & unordered communication, using WebRTC & UDP.

Utilizes the wonderful https://github.com/kyren/webrtc-unreliable


## Examples

### Server:

To run a UDP server on Linux: (that will be able to communicate with Linux clients)
    1. `cd examples/server`
    2. `cargo run --features "use-udp"``

To run a WebRTC server on Linux: (that will be able to communicate with Web clients)
    1. `cd examples/server`
    2. `cargo run --features "use-webrtc"``

### Client:

To run a UDP client on Linux: (that will be able to communicate with a UDP server)
    1. `cd examples/client`
    2. `cargo run`

To run a WebRTC client on Web: (that will be able to communicate with a WebRTC server)
    1. Enter in your IP Address at the appropriate spot in examples/client/src/app.rs
    2. `cd examples/client`
    3. `npm install` //should only need to do this once to install dependencies
    4. `npm run start` //this will open a web browser, and hot reload


To simply build these examples instead of running them, substitute
    `cargo build` for `cargo run`, and
    `npm run build` for `npm run start`