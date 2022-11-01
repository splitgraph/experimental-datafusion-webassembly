# compile library using datafusion for target `wasm32-wasi`

this compiles and runs on wasmedge

the starting point was this gist from Nov 2021: https://gist.github.com/roee88/91f2b67c3e180fa0dfb688ba8d923dae

```bash
❯ docker run --rm -it -v $(pwd)/target/wasm32-wasi/debug:/app wasmedge/slim:0.11.2-rc.1 wasmedge --reactor dfwasm.wasm _start
+---+----+
| a | b  |
+---+----+
| b | 10 |
| c | 10 |
+---+----+
0
```

when compiled with `--release`, the binary is `23M` (I believe that's [23 \_mega_bytes](https://www.gnu.org/software/coreutils/manual/html_node/Block-size.html), but it's a touchy subject):

```
❯ BLOCK_SIZE=human-readable du -bh target/wasm32-wasi/release/dfwasm.wasm
23M     target/wasm32-wasi/release/dfwasm.wasm
```

the development binary is slightly bigger:

```
❯ du -bh target/wasm32-wasi/debug/dfwasm.wasm
497M    target/wasm32-wasi/debug/dfwasm.wasm
```

it requires some minor patching of `datafusion`:

- only use `InMemory` object store
- remove two calls to `spawn_blocking` in `sort.rs` (replace with `spawn`) (this might cause some runtime logic errors)
- use latest version of arrow on github, which is later than datafusion uses. I had success with [`87ac05bcafd343d3d8ad3b519631d83090afeb1c`](https://github.com/apache/arrow-rs/commit/87ac05bcafd343d3d8ad3b519631d83090afeb1c), and then 12 minutes later they pushed the new `26.0.0` major release with ~2 commits that I don't have here
  - change `timestamp_ns_to_datetime` to `unwrap` due to new arrow API
- no bzip2
- use `std::collections::HashMap` instead of `ahash::HashMap` because for some reason there was a compilation error due to new version of ahash having different API (not sure why it wasn't an error before, maybe some transitive dependency was artificially narrowing the semver range but not anymore)

in general, most of the changes are feature-flagged and in theory this could be cleaned up and submitted as a patch to datafusion

## install pre-requisities

notes for ubuntu 20 YMMV

```bash
sudo apt-get update -qq && sudo apt-get install -yy cmake
```

we also need the `protobuf-compiler` package, but not the one from ubuntu. we need the latest

```bash
cd $(mktemp -d) && curl -L 'https://github.com/protocolbuffers/protobuf/releases/download/v21.9/protoc-21.9-linux-x86_64.zip' -o protoc.zip && unzip protoc.zip && sudo cp bin/protoc /usr/bin/ && sudo cp -r include /usr/include && cd -
```

update rust to latest stable (expected by datafusion)

```bash
rustup update stable
```

add wasm target

```bash
rustup target add wasm32-wasi
```

here are my versions at time of writing (recently successful compilation and run):

```bash
❯ rustc -vV
rustc 1.64.0 (a55dd71d5 2022-09-19)
binary: rustc
commit-hash: a55dd71d5fb0ec5a6a3a9e8c27b2127ba491ce52
commit-date: 2022-09-19
host: x86_64-unknown-linux-gnu
release: 1.64.0
LLVM version: 14.0.6

❯ rustup --version
rustup 1.25.1 (bb60b1e89 2022-07-12)
info: This is the version for the rustup toolchain manager, not the rustc compiler.
info: The currently active `rustc` version is `rustc 1.64.0 (a55dd71d5 2022-09-19)`

❯ cargo --version
cargo 1.64.0 (387270bc7 2022-09-16)

❯ protoc --version
libprotoc 3.21.9

❯ clang --version
clang version 10.0.0-4ubuntu1
Target: x86_64-pc-linux-gnu
Thread model: posix
InstalledDir: /usr/bin

❯ uname -a
Linux miles-dev-0-dev 5.15.0-1021-gcp #28~20.04.1-Ubuntu SMP Mon Oct 17 11:37:54 UTC 2022 x86_64 x86_64 x86_64 GNU/Linux

❯ lsb_release -a
No LSB modules are available.
Distributor ID: Ubuntu
Description:    Ubuntu 20.04.2 LTS
Release:        20.04
Codename:       focal
```

## setup patch for `datafusion`

clone `datafusion` from `784f10bb57f86a4db2e01a6cb51da742af0dd9d9` in `../../` (should be addressable from this directory as `../../arrow-datafusion`)

```
cd ../../
git clone https://github.com/apache/arrow-datafusion
cd arrow-datafusion
git checkout 784f10bb57f86a4db2e01a6cb51da742af0dd9d9
```

while still in the `arrow-datafusion` repo, apply the patch:

```
git apply ../rustmonke/dfwasm/datafusion.patch
```

## sanity check: build datafusion by itself

while still in the `arrow-datafusion` directory, run this command to build it with the equivalent features that we enable from our project

```bash
export RUSTFLAGS="--cfg tokio_unstable" ; cargo build --verbose --target wasm32-wasi --package datafusion --package datafusion-expr --package datafusion-proto --package datafusion-common --no-default-features --features datafusion-proto/default,datafusion/regex_expressions,datafusion/unicode_expressions,datafusion/object_store,datafusion-common/apache-avro,datafusion-common/parquet
```

it should build

## build this project

in the current directory (`dfwasm`):

```bash
export RUSTFLAGS="--cfg tokio_unstable" ; cargo build --verbose --target wasm32-wasi
```

## run it

```bash
❯ docker run --rm -it -v $(pwd)/target/wasm32-wasi/debug:/app wasmedge/slim:0.11.2-rc.1 wasmedge --reactor dfwasm.wasm _start
+---+----+
| a | b  |
+---+----+
| b | 10 |
| c | 10 |
+---+----+
0
```

pog

# discussion / notes

## protoc: remove old version

You might have an old version of `protoc` on your system, maybe from
an earlier package manager install. You can remove it:

```bash
sudo apt-get --purge remove protobuf-compiler

# you might have some proto files here that it would be good to remove too
sudo rm -rf /usr/include/google
```

see:
https://github.com/protocolbuffers/protobuf#protocol-compiler-installation

releases: https://github.com/protocolbuffers/protobuf/releases

note: make sure to click "show all assets" and that you download `protoc-`, _not_ `protobuf-`

## compiling arrow by itself

it turned out that no changes to arrow was required, however if you do
check out the `arrow-rs` repository and want to build it locally, this
command should work to compile with all the same options we're using here:

```bash
cd ../../arrow-rs

export RUSTFLAGS="--cfg tokio_unstable" ; cargo build --target wasm32-wasi --package arrow --package parquet --no-default-features --features arrow/default,arrow/prettyprint,arrow/dyn_cmp_dict,parquet/arrow,parquet/async,parquet/snap,parquet/brotli,parquet/flate2,parquet/base64
```

## tokio / wasm support

- `tokio_unstable` and wasm support
  - https://docs.rs/tokio/latest/tokio/index.html#wasm-support
- PR adding support for `wasm32-wasi` (merged 7/12/22)
  - https://github.com/tokio-rs/tokio/pull/4716
- tracking issue
  - https://github.com/tokio-rs/tokio/issues/4827

## other

- tracking `zstd` wasm support

  - https://github.com/Nemo157/async-compression/issues/142

- wasmedge tokio demo

  - https://github.com/WasmEdge/wasmedge_tokio_demo/blob/main/Cargo.toml

- reqwest wasm
  - https://github.com/samdenty/reqwest/blob/master/Cargo.toml
  - (NOTE: we are using this `reqwest` fork which I forked to https://github.com/milesforks/reqwest just to change the package name)

## tips

you can usually replace `cargo build` with `cargo tree` to get a rendering of the crate dependency tree. this helps for debugging which features are enabled, e.g.:

"why is `tokio-rt-multi-thread` being included? :thinking:" (it shouldn't be)

```
cargo tree -e features,normal -itokio-rt-multi-thread
```

see: https://doc.rust-lang.org/cargo/reference/features.html#inspecting-resolved-features

## misc link dump

demo wasmedge tokio
https://github.com/WasmEdge/wasmedge_tokio_demo/blob/main/Cargo.toml

diff wasm edge tokio
https://github.com/tokio-rs/tokio/compare/master...WasmEdge:tokio:master

example dependencies patch
https://github.com/jaemk/self_update/commit/7b9d8876f6f73a70bdf95e1348279a72daad5439

tracking https://github.com/tokio-rs/tokio/issues/4827

this is where the wasi target is implemented for rust

https://github.com/rust-lang/rust/tree/master/library/std/src/sys/wasi

the thread is just https://github.com/rust-lang/rust/blob/master/library/std/src/sys/wasi/thread.rs

https://docs.rust-embedded.org/embedonomicon/custom-target.html

need to implement something like this
https://github.com/rust-lang/rust/blob/master/library/std/src/sys/unix/thread.rs

for https://github.com/rust-lang/rust/blob/master/library/std/src/sys/wasm/atomics/thread.rs

multi-threaded emscripten
https://github.com/gregbuchholz/threads
https://github.com/gregbuchholz/orbits/commit/b91b2bfae9bc2a28cb83e16d562d2d1de8fc78d7

https://rustwasm.github.io/docs/wasm-bindgen/examples/raytrace.html
https://github.com/GoogleChromeLabs/wasm-bindgen-rayon

reqwest-wasm
https://github.com/samdenty/reqwest/blob/master/Cargo.toml

https://github.com/Nemo157/async-compression/issues/142
