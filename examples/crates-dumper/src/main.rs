use anyhow::Result;
use crb::agent::Standalone;
use crb_example_crates_dumper::CratesLoader;
use crb_system::Main;

#[tokio::main]
async fn main() -> Result<()> {
    CratesLoader::new()
        .spawn()
        .into_inner()
        .join_or_signal()
        .await
}
