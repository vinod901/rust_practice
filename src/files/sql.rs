extern crate meval;
extern crate sqlparser;
use sqlparser::{dialect::GenericDialect, parser::Parser};

// just for demonstration========================================================
#[get("/sql/select")]
pub fn sql_select() -> String {
    let dialect = GenericDialect {};
    let query = "SELECT column_name FROM table1 RIGHT JOIN table2 ON table1.column_name = table2.column_name; ";
    let ast = Parser::parse_sql(&dialect, query).unwrap();
    let tree = &ast[0];
    format!("ast : {:?}", tree)
}

// SELECT a.studentid, a.name, b.total_marks FROM student a, marks b;
// SELECT a.studentid, a.name, b.total_marks FROM student a, marks b WHERE a.studentid = b.studentid AND b.total_marks > (SELECT total_marks FROM marks WHERE studentid =  'V002');
// checkout https://www.w3resource.com/sql/subqueries/understanding-sql-subqueries.php

// SELECT CompanyName,  ProductCount = (SELECT COUNT(P.id) FROM [Product] P WHERE P.SupplierId = S.Id) FROM Supplier S;
// SELECT Orders.OrderID, Customers.CustomerName, Orders.OrderDate FROM Orders;
// SELECT * FROM Customers LIMIT 3;

// i like super man!!!!
