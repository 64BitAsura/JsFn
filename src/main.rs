use std::{io::{self}};

use boa_engine::{
    Context,
    JsResult, 
    JsValue
};

use atty::Stream;

fn load_stdin() -> io::Result<String> {
    if atty::is(Stream::Stdin) {
        return Err(io::Error::new(io::ErrorKind::Other, "stdin not redirected"));
    }
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    return Ok(buffer);
}


fn main() {
    let _ = evaluate(&(load_stdin().unwrap()));
}

pub fn std_io(_: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    println!("{}", args[0].to_string(ctx).unwrap());
    return Ok(JsValue::Null);
}

pub fn evaluate(src: &str)-> Result<String, JsValue> {
    let mut context =  Context::default();
    context.register_global_function("IO",1, std_io);
    // Setup executor
    context
        .eval(src)
        .map_err(|e| JsValue::from(println!("Uncaught {}", e.display())))
        .map(|v| v.display().to_string())
}
