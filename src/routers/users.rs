use graud;
use database::service::user;
use rocket_contrib::{Json};
use bson::Document;

#[get("/users")]
pub fn all_user(_k: graud::key::KeyVerify) -> Json<Vec<Document>> {
    let remove_keys = vec![
        "password", 
        "email", 
        "initProduce", 
        "produce", 
        "join", 
        "inviteNumber", 
        "__v", 
        "lastSignup",
        "invite"
    ];

    let mut users: Vec<Document> = Vec::new();
    for mut u in user::User::get_all_user() {
        for item in remove_keys.iter() {
            u.remove(item).unwrap();
        }
        users.push(u);
    }
    Json(users)
}
