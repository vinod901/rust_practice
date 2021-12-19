extern crate csv;
extern crate sqlparser;
// use csv::Error;
// use rocket::{
//     http::ContentType,
//     request::Request,
//     response::{self, Responder, Response},
// };
use serde::{Deserialize, Serialize};
use sqlparser::ast::{SetExpr, Statement};
use sqlparser::dialect::GenericDialect;
use sqlparser::parser::Parser;
use std::time::Instant;
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

#[get("/details/<name>/<profession>")]
pub fn hello(name: String, profession: String) -> String {
    let person = Person { name, profession };
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
}
#[get("/sql")]
pub fn get_query() -> String {
    let dialect = GenericDialect {};
    let sql = "SELECT a, b FROM table_1";
    let ast = Parser::parse_sql(&dialect, sql).unwrap();
    for k in ast.iter() {
        if let Statement::Query(query) = k {
            if let SetExpr::Select(s) = &query.body {
                println!("s : {:?}", s);
            }
        }
    }
    format!("AST : {:?}", ast).to_string()
}
