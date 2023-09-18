fn main() {
    analog::init().expect("Failed to start logger");

    log::info!("Hello, world!");
}
