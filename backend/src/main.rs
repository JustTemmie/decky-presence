use config::Config;
mod config;
mod steam;
use discord_rich_presence::{activity::{Activity, Assets}, DiscordIpc, DiscordIpcClient};

pub mod recursion {
    pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
}

pub type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut config = Config::try_load().await;

    let mut client = DiscordIpcClient::new(&config.application_id)?;

    client.connect()?;

    let mut last_currently_playing = String::new();

    loop {
        let currently_playing = config.steam.currently_playing().await?;

        if currently_playing != "" {
            if last_currently_playing != currently_playing {
                config.steam.app_id = config.steam.get_game_id(&currently_playing).await?;
            }

            println!("{}, {}", last_currently_playing, currently_playing);

            println!("{}", config.steam.app_id);

            if config.artwork.steam_grid_db.enabled {
                config
                    .artwork
                    .steam_grid_db
                    .query(&currently_playing)
                    .await?;
            } else if config.artwork.steam_store_fallback && config.steam.app_id != 0 {
                config.artwork.get_image_from_store_page(config.steam.app_id).await;
            }

            client.set_activity(Activity::new().details(&currently_playing).assets(Assets::new()))?;

            last_currently_playing = currently_playing;
        } else {
            last_currently_playing = String::new();
            config.steam.app_id = 0
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}
