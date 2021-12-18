extern crate csv;
// use csv::Error;
// use rocket::{
//     http::ContentType,
//     request::Request,
//     response::{self, Responder, Response},
// };
use serde::{Deserialize, Serialize};
use std::time::Instant;
// use std::io::Cursor;
#[derive(Serialize, Deserialize, Debug)]
struct Person {
    name: String,
    profession: String,
}

#[derive(Default, Debug)]
struct Data {
    name: String,
    headers: Vec<String>,
    data: Vec<Vec<String>>,
}
// impl<'r> Responder<'r> for Data {
//     fn respond_to(self, _: &Request) -> response::Result<'r> {
//         Response::build()
//             .sized_body(Cursor::new(format!("{}", self.name)))
//             .raw_header("X-Data-Name", self.name)
//             .header(ContentType::new("application", "x-person"))
//             .ok()
//     }
// }
#[get("/details/<name>/<prof>")]
pub fn hello(name: String, prof: String) -> String {
    let person = Person {
        name: name,
        profession: prof,
    };
    println!("Name : {}", person.name);
    println!("Profession : {}", person.profession);
    format!("{:?}", person).to_string()
}
#[get("/csv/<path>")]
pub fn read_csv(path: &str) -> String {
    let now = Instant::now();
    println!("csv file : {}\n\n", path);
    let mut reader = csv::Reader::from_path(path.clone()).expect("sorry, can't open the file!");
    println!("{:?}", reader);

    let mut table: Data = Data {
        name: path.to_string(),
        headers: Vec::new(),
        data: Vec::new(),
    };
    let head_read = Instant::now();
    for head in reader.headers() {
        for h in head {
            table.headers.push(h.to_string())
        }
    }
    let head_stop = head_read.elapsed().as_micros();
    let rec_read = Instant::now();
    reader.records().for_each(|f| {
        let mut row: Vec<String> = Vec::new();
        for value in f.iter() {
            for v in value {
                row.push(v.to_string());
            }
        }
        table.data.push(row);
    });
    let rec_stop = rec_read.elapsed().as_micros();
    let stop = now.elapsed().as_micros();
    println!(
        "table \nname : {}\nheaders : {:?}\ndata : \n{:?}",
        table.name, table.headers, table.data
    );
    println!(
        "header length : {}\nnumber of records : {}",
        table.headers.len(),
        table.data.len()
    );
    format!(
        "read csv file : {}\nHeaders : {:?}\nHeader length : {}\nHeader read time : {}\nNumber of rows : {}\nRows read time : {}\nTotal duration : {}",
        path,
        table.headers,
        table.headers.len(),
        head_stop,
        table.data.len(),
        rec_stop,
        stop
    )
    .to_string()
    // table
}
