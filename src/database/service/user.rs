use bson::Document;
use database::connect::get_database;
use mongodb::db::ThreadedDatabase;
use mongodb::coll::Collection;
use utils::result::un_warp_result;
use database::service::Service;
use bson::oid::ObjectId;
use database::service::produce::Produce;
use utils::number::get_docment_number_field;

#[derive(Debug)]
pub struct User;

impl Service for User {
    const COLLECTION: &'static str = "users";
    
    fn get_db() -> Collection {
        get_database().collection(User::COLLECTION)
    }
}

impl User {
    pub fn get_all_user() -> Vec<Document> {
        let db = User::get_db();
        un_warp_result(
            un_warp_result(
                db.find(None, None)
            ).drain_current_batch()
        )
    }

    pub fn get_user(id: String) -> Option<Document> {
        let db = User::get_db();
        match db.find_one(Some(doc!{"_id": un_warp_result(ObjectId::with_string(&id))}), None) {
            Ok(user) => {
                return user;
            },
            Err(e) => {
                println!("{}", e);
                return None
            }
        }
    }

    pub fn get_user_all_produce(user: String) -> Option<Vec<ObjectId>> {
        match User::get_user(user) {
            Some(u) => {
                let mut produces: Vec<ObjectId> = Vec::new();
                for item in un_warp_result(u.get_array("produce")) {
                    produces.push(item.as_object_id().unwrap().clone());
                
                }
                produces.push(un_warp_result(u.get_object_id("initProduce")).clone());
                // None when don't have produce
                if produces.len() == 0 {
                    return None;
                }
                
                return Some(produces);
            }
            None => return None,
        }
    }

    pub fn add_traffic_to_user(user: String, traffic: i32) -> Option<()> {
        let produce_db = Produce::get_db();
        match Produce::get_user_produce(user) {
            Some(field) => {
                let mut remain_add = traffic as i64;
                for item in field.iter() {
                    let have = get_docment_number_field(item, "traffic");
                    let used = get_docment_number_field(item, "used");
                    if have > used {
                        // Use if produce is available
                        if have < used + remain_add {
                            // Produce traffic is not enough to add traffic
                            let add_with = have - used;
                            let _ = produce_db.update_one(
                                doc!{ "_id": un_warp_result(item.get_object_id("_id")).clone(), },
                                doc!{ "$inc": { "used": add_with, } },
                                None
                            );
                            remain_add -= add_with;
                        } else {
                            // Add traffic to the produce
                            let _ = produce_db.update_one(
                                doc!{ "_id": un_warp_result(item.get_object_id("_id")).clone(), },
                                doc!{ "$inc": { "used": remain_add, } },
                                None
                            );
                            return Some(());
                        }
                    }
                }
                return None;
            },
            None => {
                return None;
            }
        }
    }
}