# naia-socket

Examples

Server:

To run a UDP server on Linux: (that will be able to communicate with Linux clients)
    `cd examples/server`
    `cargo run --features "use-udp"``

To run a WebRTC server on Linux: (that will be able to communicate with Web clients)
    `cd examples/server`
    `cargo run --features "use-webrtc"``

Client:

To run a UDP client on Linux: (that will be able to communicate with a UDP server)
    `cd examples/client`
    `cargo run`

To run a WebRTC client on Web: (that will be able to communicate with a WebRTC server)
    `cd examples/server`
    `npm install` //should only need to do this once to install dependencies
    `npm run start` //this will open a web browser, and hot reload


To simply build these examples instead of running them, substitute
    `cargo build` for `cargo run`, and
    `npm run build` for `npm run start`