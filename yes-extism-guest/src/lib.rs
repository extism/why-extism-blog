use extism_pdk::*;

#[plugin_fn]
pub fn say_hello(input: String) -> FnResult<String> {
    let greeting = format!("Hello, {}", input);
    Ok(greeting)
}
