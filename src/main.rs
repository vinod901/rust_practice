#[macro_use]
extern crate rocket;
// use std::error::Error;
mod files;
use files::routes::{get_query, hello, read_csv};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, read_csv, get_query])
}
