use rocket::Outcome;
use rocket::http::Status;
use rocket::request::{self, Request, FromRequest};
use conf::Settings;
use utils::result;

pub struct KeyVerify(String);

impl<'a, 'r> FromRequest<'a, 'r> for KeyVerify {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<KeyVerify, ()> {
        match request.headers().get_one("x-key") {
            Some(key) => {
                if key == result::un_warp_result(Settings::new()).security.key {
                    return Outcome::Success(KeyVerify(key.to_string()));
                } else {
                    return Outcome::Failure((Status::new(401, "Error Key"), ()));
                }
            },
            None => {
                return Outcome::Failure((Status::BadRequest, ()))
            }
        }
    }
}