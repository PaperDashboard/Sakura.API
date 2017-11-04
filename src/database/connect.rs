use mongodb::{Client, ThreadedClient};
use utils::result::un_warp_result;
use mongodb::db::Database;
use conf::Settings;

pub fn get_database() -> Database {
    let settings = un_warp_result(Settings::new());
    
    let client = Client::connect(&settings.database.host, settings.database.port)
        .expect("Failed to initialize standalone client.");
    
    client.db(&settings.database.db)
}