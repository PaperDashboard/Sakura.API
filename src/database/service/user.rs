use bson::Document;
use database::connect::get_database;
use mongodb::db::ThreadedDatabase;
use utils::result::un_warp_result;

#[derive(Debug)]
pub struct User;

impl User {
    const COLLECTION: &'static str = "users";

    pub fn get_all_user() -> Vec<Document> {
        let db = get_database().collection(User::COLLECTION);
        un_warp_result(
            un_warp_result(
                db.find(None, None)
            ).drain_current_batch()
        )
    }
}