use result_logger::ResultLogger;

fn main() {
    simple_logger::init().unwrap();
    _ = calling_func();
}

fn calling_func() -> Result<(), String> {
    failing_func().trace()?;

    Ok(())
}

fn failing_func() -> Result<(), String> {
    Err("This is an error".to_string())
}
