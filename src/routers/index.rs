use graud;

#[get("/")]
pub fn index(_k: graud::key::KeyVerify) -> &'static str {
    "Hello, World"
}
