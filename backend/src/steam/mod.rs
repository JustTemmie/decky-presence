mod artwork;
use serde_json::Value;
use crate::config::Steam;

impl Steam {
    pub async fn currently_playing(&self) -> Result<String, crate::Error> {
            let response: Value = reqwest::get(
                format!("https://api.steampowered.com/ISteamUser/GetPlayerSummaries/v2/?key={}&format=json&steamids={}", 
                    self.api_key, 
                    self.user_id)
                ).await?.json().await?;

            let game_info = &response["response"]["players"][0]["gameextrainfo"];

            Ok(game_info.as_str().unwrap_or("").to_string())
    }

    pub async fn get_game_id(&self, currently_playing: &str) -> Result<u64, crate::Error> {
        let response: Value = reqwest::get(
            format!("https://api.steampowered.com/ISteamApps/GetAppList/v0002/?key={}&format=json", 
                self.api_key)
            ).await?.json().await?;

        if let Some(games) = response["applist"]["apps"].as_array() {
            for game in games {
                if game["name"] == currently_playing {
                    return Ok(game["appid"].as_u64().unwrap())
                }
            }
        }

        Ok(0)
    }
}
