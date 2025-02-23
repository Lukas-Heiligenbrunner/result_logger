# result_logger

Rust crate to ease the logging of error results.

Just call the provided log funtions on Results and if its an error it gets logged by your logging implementation.

## Available functions



## Usage

```rust
fn calling_func() -> Result<(), String> {
    failing_func().error()?;
    
    // Do something
    
    Ok(())
}

fn failing_func() -> Result<(), String> {
    Err("This is an error".to_string())
}
```
-> This will log the error message "This is an error" with the log level Error.

## License

This project is licensed under the MIT License. Feel free to contribute and modify as per the guidelines outlined in the license agreement.
