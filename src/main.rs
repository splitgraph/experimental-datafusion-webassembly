// run via library with --reactor
// see: https://wasmedge.org/book/en/quick_start/run_cli.html
/*

docker run --rm -it -v $(pwd)/target/wasm32-wasi/debug:/app \
    wasmedge/slim:0.11.2-rc.1 wasmedge --reactor dfwasm.wasm _start

*/

fn main() {
    println!("compiled as a library at the moment");
}
