use std::error::Error;

mod server;

fn main() -> Result<(), Box<dyn Error>> {
    server::run()
}
