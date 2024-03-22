use config::Config;
mod config;
mod steam;

pub mod recursion {
    pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
}

pub type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let config = Config::try_load().await;

    loop {
        config.steam.currently_playing().await?;

        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }
}
