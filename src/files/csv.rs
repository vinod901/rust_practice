extern crate csv;
use csv::Error;
#[allow(dead_code)]
pub fn read_csv(path: &str) -> Result<(), Error> {
    println!("csv file : {}\n\n", path);
    let mut reader = csv::Reader::from_path(path).expect("sorry, can't open the file!");
    println!("{:?}", reader);
    let headers = reader.headers();
    println!("{:?}", headers);

    // let mut reader2 = csv::Reader::from_path("players.csv")?;
    // for result in reader2.records() {
    //     let record = result?;
    //     println!("{:?}", record);
    // }
    reader
        .records()
        .for_each(|f| println!("col-1 : {}", &f.unwrap()[0]));
    // reader.records().for_each(|f| {
    //     println!(
    //         "col-1 : {} col-2 : {}",
    //         &f.as_ref().unwrap()[0],
    //         &f.as_ref().unwrap()[9]
    //     )
    // });
    Ok(())
}
