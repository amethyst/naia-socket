# README #

Some notes:
To run on the web, you gotta do:
1. `rustup target add wasm32-unknown-unknown`
1. `cargo install cargo-web`

----

New plan:

1. Implement Udp Server / Client sockets using Laminar dependency
2. Pull Laminar dependency into Gaia, keep your test case working...
3. Slowly remove stuff, keep your test case working...
        arranging, sequencing, fragmenting, reliable, testing framework
4. rename to udp_library, something like that

// WebRTC
1. Get example working again, to understand how to run it
2. Get client working by itself - no serving it from the server, need to run independantly
3. Try to get example working with the server being initialized from Gaia
4. Then, using wasm-bindgen, try to get the client part working from Gaia


//DONE!