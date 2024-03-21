#[derive(serde::Deserialize, serde::Serialize)]
pub struct Config {
    pub steam: Steam,
    pub discord: Discord,
    pub steam_grid_db: SteamGridDb,
}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Steam {}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct Discord {}

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SteamGridDb {}

impl Config {
    async fn load() -> Result<Config, crate::Error> {
        let file = tokio::fs::read_to_string("./config.json").await?;
        let config: Config = serde_json::from_str(&file)?;

        Ok(config)
    }
}
