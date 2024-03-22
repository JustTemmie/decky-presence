use crate::config::SteamGridDb;
use crate::Error;
use steamgriddb_api::images::Image;
use steamgriddb_api::Client;
use steamgriddb_api::QueryType::Grid;

impl SteamGridDb {
    async fn client(&self) -> Result<Client, Error> {
        Ok(Client::new(self.api_key.clone()))
    }

    pub async fn query(&self, game: &str) -> Result<Option<&Image>, Error> {
        let client = self.client().await?;
        let games = client.search(game).await?;
        let first_game = games.iter().next();
        Ok(first_game.and_then(|first_game| async {
            let images = client.get_images_for_id(first_game.id, &Grid(None)).await?;
            images.iter().next()
        }))
    }
}
