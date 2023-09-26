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

    // get the guest's exported functions
    let say_hello = instance.get_typed_func::<(i32, i32, i32), i32>(&mut store, "say_hello")?;
    let alloc = instance.get_typed_func::<(), i32>(&mut store, "alloc")?;
    let dealloc = instance.get_typed_func::<i32, ()>(&mut store, "dealloc")?;

    // get an instance of linear memory
    let memory = instance
        .get_memory(&mut store, "memory")
        .ok_or(anyhow::format_err!("failed to find `memory` export"))?;

    // allocate memory to hold the input string and the output string return length
    let input_addr = alloc.call(&mut store, ())?;
    let output_len_addr = alloc.call(&mut store, ())?;

    // define the input string and write the string as binary to the memory we allocated for the input string
    let input_string = b"non-extism host!";
    memory.write(&mut store, input_addr as usize, input_string)?;

    // invoke the `say_hello` function and save the memory address to where the output will be saved
    let output_addr = say_hello.call(
        &mut store,
        (
            input_addr,                // input memory address
            input_string.len() as i32, // input length
            output_len_addr as i32,    // output length memory address
        ),
    )?;

    // we are storing the length of the output string as an unsigned 32 bit integer in linear memory, so let's
    // allocate 4 bytes, read the data from linear memory, and store it in the len_buffer
    let mut len_buffer = [0u8; 4];
    memory.read(&store, output_len_addr as usize, &mut len_buffer)?;

    // convert the bytes into an i32 (WebAssembly is always little endian)
    let len = i32::from_le_bytes(len_buffer);

    // let's now create a new Vec<u8> to store the bytes of the output string and fit it to size
    let mut v = Vec::<u8>::new();
    v.resize(len as usize, 0);

    // read the memory address returned to us by the `say_hello` function, which points to the location
    // in linear memory where our WebAssembly module stored the output string
    memory.read(&store, output_addr.try_into().unwrap(), &mut v)?;

    // decode the byte vector into a UTF8 string and print to the console!
    println!("{}", String::from_utf8_lossy(&v));

    // deallocate all of the memory that we allocated
    // this is technically unnecessary because the program is about to end, but that may not always be the case
    dealloc.call(&mut store, input_addr)?;
    dealloc.call(&mut store, output_len_addr)?;

    Ok(())
}
