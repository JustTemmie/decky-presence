use async_recursion::async_recursion;

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub steam: Steam,
    pub application_id: String,
    pub artwork: Artwork,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Steam {
    pub api_key: String,
    pub user_id: String,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Artwork {
    pub steam_grid_db: SteamGridDb,
    pub steam_store_fallback: bool,
}

#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SteamGridDb {
    pub enabled: bool,
    pub api_key: String,
}

impl Config {
    #[async_recursion]
    pub async fn try_load() -> Config {
        match Config::load().await {
            Ok(config) => config,
            Err(e) => {
                println!("{:#?}", e);
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                Config::try_load().await
            }
        }
    }

    pub async fn load() -> Result<Config, crate::recursion::Error> {
        let file = tokio::fs::read_to_string("./config.json").await?;
        let config_builder: ConfigBuilder = serde_json::from_str(&file)?;

        let config = config_builder.build().await;

        println!("{:#?}", config);
        Ok(config)
    }
}

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct ConfigBuilder {
    steam: Steam,
    application_id: Option<String>,
    artwork: Option<ArtworkBuilder>,
}

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtworkBuilder {
    steam_grid_db: Option<SteamGridDbBuilder>,
    steam_store_fallback: Option<bool>,
}

#[derive(Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct SteamGridDbBuilder {
    enabled: Option<bool>,
    api_key: Option<String>,
}

impl ConfigBuilder {
    async fn build(mut self) -> Config {
        if self.application_id.is_none() {
            self.application_id = Some(String::from("869994714093465680"));
        }
        
        let artwork = if let Some(mut artwork) = self.artwork {
            if let Some(mut steam_grid_db) = artwork.steam_grid_db {
                if steam_grid_db.enabled.is_none() {
                    steam_grid_db.enabled = Some(false);
                }

                if steam_grid_db.api_key.is_none() {
                    steam_grid_db.api_key = Some(String::new())
                }

                artwork.steam_grid_db = Some(steam_grid_db);
            } else {
                artwork.steam_grid_db = {
                    Some(SteamGridDbBuilder {
                        enabled: Some(false),
                        api_key: Some(String::new()),
                    })
                }
            }

            if artwork.steam_store_fallback.is_none() {
                artwork.steam_store_fallback = Some(false);
            }

            artwork
        } else {
            ArtworkBuilder {
                steam_grid_db: Some(SteamGridDbBuilder {
                    enabled: Some(false),
                    api_key: Some(String::new()),
                }),
                steam_store_fallback: Some(false),
            }
        };

        self.artwork = Some(artwork);

        Config {
            steam: self.clone().steam,
            application_id: self.clone().application_id.unwrap(),
            artwork: Artwork {
                steam_grid_db: SteamGridDb {
                    enabled: self.artwork().steam_grid_db().enabled.unwrap(),
                    api_key: self.artwork().steam_grid_db().api_key.unwrap(),
                },
                steam_store_fallback: self.artwork().steam_store_fallback.unwrap(),
            }
        }
    }

    fn artwork(&self) -> ArtworkBuilder {
        self.artwork.clone().unwrap()
    }
}

impl ArtworkBuilder {
    fn steam_grid_db(&self) -> SteamGridDbBuilder {
        self.steam_grid_db.clone().unwrap()
    }
}
