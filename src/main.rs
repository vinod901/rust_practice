// #![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
// use std::error::Error;
mod files;
use files::routes::{hello, read_csv};

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![hello, read_csv])
}
