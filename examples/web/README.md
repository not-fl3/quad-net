quad-net.js is a minified version of /js/quad-net.js  
gl-0.1.19.js is a hosted version of https://not-fl3.github.io/miniquad-samples/gl-0.1.19.js  
sapp-utils.js is a hosted version of https://not-fl3.github.io/miniquad-samples/sapp-jsutils.js  

To build client.wasm:
```
cargo build --example client --target wasm32-unknown-unknown --release 
cp ../../target/wasm32-unknown-unknown/release/examples/client.wasm . 

# step to host index.html
# or any other web server familiar with wasm MIME
basic-http-server .
```

To run server: 
```
cargo run --example server
```

To run native client: 
```
cargo run --example client
```
