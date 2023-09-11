# Things I want to have a look regarding the async transformation

## Nice reads

* https://github.com/tessi/wasmex/issues/394
* https://github.com/tessi/wasmex/issues/391 (esp joses points about async and reaching 100k concurrent calls)
    * https://github.com/tessi/wasmex/issues/391#issuecomment-1374444104
* https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html#method.async_support
* https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html#method.epoch_interruption

## TODOs

* fix any //TODO: comment in the code
* make any and all time consuming NIF calls async
* add a new config to limit/inspect CPUs used by tokio
    * would be nice to have a thread pool configured per WASM instance and not for the whole NIF, not sure yet
    * also not sure if this should be a compile-time config (on NIF load) or changeable at runtime
* async NIF calls should receive a PID to answer to (so we don't hardcode this to the wasmex-GenServer) - this allows to call/receive messages across Erlang nodes
* make WASM function calls async  with wasmtime, configure wasmtime to be async (Config.async_support)
* enable setting of epoch_based interruption of WASM function calls
    * this allows us to run many WASM function calls with a limited number of CPUs (cooperative timeslicing in long running WASM calls) - see https://docs.rs/wasmtime/latest/wasmtime/struct.Config.html#method.epoch_interruption
    * also protects against exploiting WASM binaries by being able to interrupt too-long running calls (in addition to fuel, since fuel calculation is slow)
    * we might want to add this at a later point, but maybe prototype it - having this feature is the main reason behind async
* cover all the things with tests
* docs! (also upgrade docs)
