use crate::config::{Artwork, SteamGridDb};
use crate::Error;
use serde_json::Value;
use steamgriddb_api::Client;
use steamgriddb_api::QueryType::Grid;

impl SteamGridDb {
    async fn client(&self) -> Result<Client, Error> {
        Ok(Client::new(self.api_key.clone()))
    }
}

impl Artwork {
    pub async fn get_image_from_store_page(&mut self, app_id: u64) -> Result<(), Error> {
        let response: Value = reqwest::get(format!("https://store.steampowered.com/api/appdetails?appids={}", app_id)).await?.json().await?;
        self.image_url = response[app_id.to_string()]["data"]["header_image"].as_str().unwrap().to_string();

        Ok(())
    }

    pub async fn sgdb_query(&mut self, game: &str) -> Result<(), Error> {
        let client = self.steam_grid_db.client().await?;
        let games = client.search(game).await?;
        let first_game = games.iter().next();

        match first_game {
            Some(game) => Ok({
                let images = client.get_images_for_id(game.id, &Grid(None)).await?;

                let image = images.iter().next();

                match image {
                    Some(image) => {
                        self.image_url = image.url.clone();
                    },
                    None => (),
                }
            }),
            None => Ok(()),
        }
    }
}
