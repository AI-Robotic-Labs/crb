use crb::agent::Standalone;
use web_app::Frontend;

fn main() {
    // TODO: Is that possible to use `tracing` package here?
    let config = wasm_logger::Config::new(log::Level::Warn);
    wasm_logger::init(config);
    log::info!("WEB APP LOADED");
    Frontend::new().spawn();
}
