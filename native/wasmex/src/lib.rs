mod atoms;
mod caller;
mod engine;
mod environment;
mod functions;
mod instance;
mod memory;
mod module;
mod pipe;
mod printable_term_type;
mod store;
mod store_limits;
mod task;

#[macro_use]
extern crate rustler;

use rustler::{Env, Term};

rustler::init! {
    "Elixir.Wasmex.Native",
    [
        engine::new,
        engine::precompile_module,
        instance::call_exported_function,
        instance::function_export_exists,
        instance::new,
        instance::receive_callback_result,
        memory::from_instance,
        memory::get_byte,
        memory::grow,
        memory::read_binary,
        memory::set_byte,
        memory::size,
        memory::write_binary,
        module::compile,
        module::exports,
        module::imports,
        module::name,
        module::serialize,
        module::unsafe_deserialize,
        pipe::new,
        pipe::read_binary,
        pipe::seek,
        pipe::size,
        pipe::write_binary,
        store::add_fuel,
        store::consume_fuel,
        store::fuel_consumed,
        store::new_wasi,
        store::new,
    ],
    load = on_load
}

fn on_load(env: Env, _info: Term) -> bool {
    rustler::resource!(engine::EngineResource, env);
    rustler::resource!(environment::CallbackTokenResource, env);
    rustler::resource!(instance::InstanceResource, env);
    rustler::resource!(memory::MemoryResource, env);
    rustler::resource!(module::ModuleResource, env);
    rustler::resource!(pipe::PipeResource, env);
    rustler::resource!(store::StoreOrCallerResource, env);
    true
}
