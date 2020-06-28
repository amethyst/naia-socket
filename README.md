# naia-socket

Examples

To run a UDP server on Linux: (that will be able to communicate with Linux clients)
    cd examples/server
    cargo run --features "use-udp"

To run a WebRTC server on Linux: (that will be able to communicate with Web clients)
    cd examples/server
    cargo run --features "use-webrtc"