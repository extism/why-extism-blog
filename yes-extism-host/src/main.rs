fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ctx = extism::Context::new();
    let mut plugin = extism::Plugin::new(
        &ctx,
        include_bytes!(
            "../../yes-extism-guest/target/wasm32-unknown-unknown/debug/yes_extism.wasm"
        ),
        [],
        false,
    )?;
    let data = plugin.call("say_hello", "extism host!")?;
    println!("{}", String::from_utf8_lossy(data));
    Ok(())
}
