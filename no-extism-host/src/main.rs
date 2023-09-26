use wasmtime::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // checkout the wasmtime crate docs to learn more about embedding wasmtime into your rust applications - https://docs.wasmtime.dev/examples-rust-embed.html
    let engine = Engine::default();
    let mut store = Store::new(&engine, ());
    let module = Module::new(
        &engine,
        // the location of the Wasm binary we compiled from our guest code
        include_bytes!(
            "../../no-extism-guest/target/wasm32-unknown-unknown/debug/no_extism_guest.wasm"
        ),
    )?;
    let instance = Instance::new(&mut store, &module, &[])?;

    // get an instance of Linear Memory
    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;

    let input_string = b"non-extism host!";
    // write the string as binary to location 0 in Linear Memory
    memory.write(&mut store, 0, input_string)?;

    // get our TypedFunction, `say_hello` from the WebAssembly exports
    let say_hello = instance.get_typed_func::<(i32, i32, i32), i32>(&mut store, "say_hello")?;

    // invoke the `say_hello` function
    let ptr = say_hello.call(
        &mut store,
        (
            0,                               // input memory address
            input_string.len() as i32,       // input length
            (input_string.len() + 1) as i32, // output length memory address
        ),
    )?;

    // we are storing the length of the output string as an unsigned 32 bit integer in Linear Memory, so let's
    // allocate 4 bytes, read the data from Linear Memory, and store it in the len_buffer
    let mut len_buffer = [0u8; 4];
    memory.read(&store, input_string.len() + 1, &mut len_buffer)?;

    // now convert the bytes into an i32 (WebAssembly is always little endian)
    let len = i32::from_le_bytes(len_buffer);

    // let's now create a new Vec<u8> to store the bytes of the output string and fit it to size
    let mut v = Vec::<u8>::new();
    v.resize(len as usize, 0);

    // read the memory address returned to us by the `say_hello` function, which points to the location
    // in Linear Memory where our WebAssembly module stored the output string
    memory.read(&store, ptr.try_into().unwrap(), &mut v)?;

    // decode the byte vector into a UTF8 string and print to the console!
    println!("{}", String::from_utf8_lossy(&v));

    Ok(())
}
