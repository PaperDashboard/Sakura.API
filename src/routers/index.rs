use graud;

#[get("/")]
pub fn index(k: graud::key::KeyVerify) -> &'static str {
    "Hello, World"
}
