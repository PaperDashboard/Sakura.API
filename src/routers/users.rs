use graud;
use database::service::user;
use rocket_contrib::{Json, Value};
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
            u.remove(item);
        }
        users.push(u);
    }
    Json(users)
}


#[derive(Debug, Deserialize)]
pub struct PostValue {
    value: i32,
}

#[put("/<user_id>/addTraffic", format = "application/json", data = "<data>")]
pub fn add_traffic(
        _k: graud::key::KeyVerify, 
        data: Json<PostValue>,
        user_id: String
    ) -> Json<Value> {
    

    match user::User::add_traffic_to_user(user_id, data.value) {
        Some(_) => {
            Json(json!({    
                "status": "ok"
            }))
        }
        None => {
            Json(json!({
                "status": "error",
                "error": "User add traffic is failure"
            }))
        }
    }
}