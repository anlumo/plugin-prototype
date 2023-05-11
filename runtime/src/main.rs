use wasmer::{
    imports, Function, FunctionEnv, FunctionEnvMut, Instance, Memory, Module, Store, Value, WasmPtr,
};

const PLUGIN: &[u8] = include_bytes!("../../target/wasm32-unknown-unknown/debug/add_plugin.wasm");

fn log_line(mut env: FunctionEnvMut<Env>, text: WasmPtr<u8>, len: u32) {
    let (data, store) = env.data_and_store_mut();
    let mem_view = data.memory.as_ref().unwrap().view(&store);
    let text = text
        .read_utf8_string(&mem_view, len)
        .expect("Requires utf-8 string");

    println!("[log] {text}");
}

struct Env {
    memory: Option<Memory>,
}

fn main() -> anyhow::Result<()> {
    let mut store = Store::default();
    let module = Module::new(&store, PLUGIN)?;

    let env = FunctionEnv::new(&mut store, Env { memory: None });

    let import_object = imports! {
        "env" => {
            "log_line" => Function::new_typed_with_env(&mut store, &env, log_line),
        },
    };
    let instance = Instance::new(&mut store, &module, &import_object)?;

    let env_mut = env.as_mut(&mut store);
    let memory = instance.exports.get_memory("memory")?;
    env_mut.memory = Some(memory.clone());

    let add = instance.exports.get_function("add")?;

    let result = add.call(&mut store, &[Value::F32(1.0), Value::F32(4.0)])?;

    println!("1.0 + 4.0 + {result:?}");

    Ok(())
}
