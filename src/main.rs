#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

mod conf;
mod utils;
mod graud;
mod routers;

fn main() {
    match conf::Settings::new() {
        Ok(_) => { },
        Err(e) => {
            println!("{}", e);
            return;
        }
    }
    
    rocket::ignite()
        // .attach(fairing::key::KeyVerify::default())
        .mount("/", routes![routers::index::index])
        .launch();
}
