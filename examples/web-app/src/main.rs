use crb::agent::Runnable;
use web_app::Frontend;
use tokio_with_wasm::alias as tokio;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    console_error_panic_hook::set_once();
    // TODO: Is that possible to use `tracing` package here?
    let config = wasm_logger::Config::new(log::Level::Info);
    wasm_logger::init(config);
    log::info!("WEB APP LOADED");
    Frontend::new().run().await;
}
