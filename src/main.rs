#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate rocket_contrib;
extern crate config;
extern crate serde;

#[macro_use]
extern crate serde_derive;

#[macro_use(bson, doc)]
extern crate bson;
extern crate mongodb;

mod conf;
mod utils;
mod graud;
mod routers;
mod database;

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
        .mount("/", routes![
            routers::index::index,
            routers::users::all_user,
        ])
        .launch();
}
