fn main() {
    analog::init(log::LevelFilter::Trace);

    log::trace!("Trace");
    log::debug!("Debug");
    log::info!("Info");
    log::warn!("Warn");
    log::error!("Error");
}
