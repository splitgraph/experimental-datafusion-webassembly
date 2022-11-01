This repository is a proof-of-concept / experiment / demo of compiling a library
using [Apache DataFusion](https://github.com/apache/arrow-datafusion) targeting WebAssembly.

There are two branches:

* `wasm32-wasi` which compiles and runs in `wasmedge`
* `wasm32-unknown-unknown` which compiles and runs in the browser

Navigate to those branches to see the demo code and readme with full details.

I got `wasm32-wasi` working first. See the readme in that branch
for all the details. Then, I branched off to `wasm32-unknown-unknown`
and got that working. See the difference between the two branches
for which changes were required for `wasm32-unknown-unknown`.

Note: the `wasm32-unknown-unknown` branch is messier than the
`wasm32-wasi` branch, and it's likely that all the patches 
it includes are not actually required to get compilation working.

Ultimately, these two branches could be cleaned up and submitted
together as an upstream patch to datafusion.


-------

For more details, see my 
[comment on the datafusion tracking issue for wasm support](https://github.com/apache/arrow-datafusion/issues/177#issuecomment-1297918179)
explaining the contents of this patch.

