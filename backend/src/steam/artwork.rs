use crate::config::{Artwork, SteamGridDb};
use crate::Error;
use steamgriddb_api::images::Image;
use steamgriddb_api::Client;
use steamgriddb_api::QueryType::Grid;

impl SteamGridDb {
    async fn client(&self) -> Result<Client, Error> {
        Ok(Client::new(self.api_key.clone()))
    }

    pub async fn query(&self, game: &str) -> Result<Option<Image>, Error> {
        let client = self.client().await?;
        let games = client.search(game).await?;
        let first_game = games.iter().next();

        match first_game {
            Some(game) => Ok({
                let images = client.get_images_for_id(game.id, &Grid(None)).await?;

                let image = images.iter().next();

                match image {
                    Some(image) => {
                        Some(image.to_owned())
                    },
                    None => None,
                }
            }),
            None => Ok(None),
        }
    }
}

impl Artwork {
    pub async fn get_image_from_store_page(&self, app_id: u64) {

        reqwest::get(format!("https://store.steampowered.com/api/appdetails?appids={}", app_id)).await;
    }
}
