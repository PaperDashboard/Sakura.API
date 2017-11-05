pub mod user;
pub mod produce;

use mongodb::coll::Collection;

trait Service {
    const COLLECTION: &'static str;

    fn get_db() -> Collection;
}