mod config;
mod steam;

pub type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    Ok(())
}
