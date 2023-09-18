# Inlog - An incremental logger.

Analog is a new logger meant for simple programs that need basic `log` implementation.

## Usage:
```rust
fn main() {
    init(log::LevelFilter::Log);

    log::info!("It works!");
    log::debug!("It debugs!");
}
```
