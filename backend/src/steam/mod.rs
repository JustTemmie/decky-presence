mod artwork;
use serde_json::Value;
use crate::config::Steam;

impl Steam {
    pub async fn currently_playing(&self) -> Result<(), crate::Error> {
            let response: Value = reqwest::get(
                format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&format=json&steamids={}", 
                    self.api_key, 
                    self.user_id)
                ).await?.json().await?;

            let game_info = &response["response"]["players"][0]["gameextrainfo"];

            println!("{}", game_info);

            Ok(())
    }
}
