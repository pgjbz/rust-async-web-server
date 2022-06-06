use std::error::Error;

mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    server::run().await
}
