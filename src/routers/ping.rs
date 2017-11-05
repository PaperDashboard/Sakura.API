use rocket_contrib::{Json, Value};

#[get("/ping")]
pub fn ping() -> Json<Value> {
    Json(json!({
        "pong": true,
    }))
}