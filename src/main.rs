#[macro_use]
extern crate rocket;
mod files;
use files::{
    fold::fold,
    routes::{get_query, hello, read_csv, send_data},
    sql::sql_select,
};

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/",
        routes![hello, read_csv, get_query, send_data, fold, sql_select],
    )
}
