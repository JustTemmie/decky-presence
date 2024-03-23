use config::Config;
mod config;
mod steam;
use discord_rich_presence::{
    activity::{Activity, Assets, Timestamps},
    DiscordIpc, DiscordIpcClient,
};
use serde_json::Value;

pub mod recursion {
    pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
}

pub type Error = Box<dyn std::error::Error>;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let mut config = Config::try_load().await;

    let mut client = DiscordIpcClient::new(&config.application_id)?;

    let mut last_currently_playing = String::new();
    let mut start_time: u64 = 0;
    let mut application_id: String;

    loop {
        let currently_playing = config.steam.currently_playing().await?;

        if !currently_playing.is_empty() {
            if last_currently_playing != currently_playing {
                application_id = get_discord_app(&currently_playing, config.application_id.to_owned()).await.unwrap();
                config.steam.app_id = config.steam.get_game_id(&currently_playing).await?;
                start_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();

                client = DiscordIpcClient::new(&application_id).expect("Failed to create Discord RPC client, discord is down or the Client ID is invalid.");

                loop {
                    match client.connect() {
                        Ok(_) => (),
                        Err(_) => {
                            println!("Failed to connect, retrying in 10 seconds"); 
                            std::thread::sleep(std::time::Duration::from_secs(10)); 
                            continue
                        },
                    };
                    break;
                }
            }

            if config.artwork.steam_grid_db.enabled {
                config.artwork.sgdb_query(&currently_playing).await?;
            } else if config.artwork.steam_store_fallback && config.steam.app_id != 0 {
                config
                    .artwork
                    .get_image_from_store_page(config.steam.app_id)
                    .await?;
            } else {
                config.artwork.image_url = String::from("https://raw.githubusercontent.com/JustTemmie/steam-presence/main/readmeimages/defaulticon.png");
            }

            client.set_activity(
                Activity::new()
                    .details(&currently_playing)
                    .timestamps(Timestamps::new().start(start_time as i64))
                    .assets(Assets::new().large_image(&config.artwork.image_url)),
            )?;

            last_currently_playing = currently_playing;
        } else {
            last_currently_playing = String::new();
            config.steam.app_id = 0;
            config.artwork.image_url = String::new();
            start_time = 0;
        }

        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    }
}

async fn get_discord_app(query: &str, application_id: String) -> Result<String, reqwest::Error> {
    let response: Vec<Value> = reqwest::get("https://discordapp.com/api/v8/applications/detectable").await?.json().await?;

    // Get the response from the json
    let mut id: String = application_id;
    for app in response {
        if app["name"].as_str().unwrap().contains(query) {
            id = app["id"].as_str().unwrap().to_string()
        }
    }
    Ok(id)
}
