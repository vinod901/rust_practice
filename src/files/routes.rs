extern crate csv;
extern crate sqlparser;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};
use sqlparser::{
    ast::{SetExpr, Statement},
    dialect::GenericDialect,
    parser::Parser,
};
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Person {
    name: String,
    profession: String,
}

#[derive(Default, Debug)]
struct Info {
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

    let mut table: Info = Info {
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
        "read csv file : {}\nHeaders : {:?}\nHeader length : {}\nHeader read time : {} micro seconds\nNumber of rows : {}\nRows read time : {} micro seconds\nTotal duration : {} micro seconds",
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
    let sql = "SELECT * FROM table_1";
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

#[post("/post", data = "<data>")]
pub fn send_data(data: Json<Person>) -> String {
    format!("I am {}. i am a {}!", data.name, data.profession).to_string()
    // serde_json::to_string(&data).unwrap()
}
