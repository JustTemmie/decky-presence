use config::Config;

mod config;
mod steam;

pub type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::load().await?;

    config
        .artwork
        .steam_grid_db
        .query("Beaver Clicker")
        .await?;

    Ok(())
}
