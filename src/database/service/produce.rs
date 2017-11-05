use bson::Document;
use database::service::user::User;
use utils::result::un_warp_result;
use database::connect::get_database;
use mongodb::db::ThreadedDatabase;
use database::service::Service;
use mongodb::coll::Collection;
use bson::oid::ObjectId;

#[derive(Debug)]
pub struct Produce;

impl Service for Produce {
    const COLLECTION: &'static str = "produces";

    fn get_db() -> Collection {
        get_database().collection(Produce::COLLECTION)
    }
}

impl Produce {
    pub fn get_produce(id: String) -> Option<Document> {
        let db = Produce::get_db();
        match db.find_one(Some(doc!{"_id": un_warp_result(ObjectId::with_string(&id))}), None) {
            Ok(produce) => {
                return Some(produce.unwrap());
            }
            Err(e) => {
                println!("{}", e);
                return None;
            }
        }
    }

    pub fn get_user_produce(id: String) -> Option<Vec<Document>> {
        match User::get_user_all_produce(id) {
            Some(produces) => {
                let mut ret: Vec<_> = Vec::new();
                for item in produces.iter() {
                    ret.push(Produce::get_produce(item.to_hex()).unwrap())
                }
                return Some(ret);
            },
            None => None,
        }
    }
}