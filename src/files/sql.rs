extern crate meval;
extern crate sqlparser;
use sqlparser::{
    ast::{Expr, ObjectName, Select, SelectItem, SetExpr, Statement, TableFactor},
    dialect::GenericDialect,
    parser::Parser,
};

#[derive(Debug)]
struct ColInfo {
    name: String,
    alias: Option<String>,
    fun: Option<String>,
}
impl ColInfo {
    fn new() -> ColInfo {
        ColInfo {
            name: "".to_string(),
            alias: None,
            fun: None,
        }
    }
}
// just for demonstration========================================================
#[get("/sql/select")]
pub fn sql_select() -> String {
    let dialect = GenericDialect {};
    let query = "SELECT * FROM Customers LIMIT 3;";
    let mut ast = Parser::parse_sql(&dialect, query).unwrap();
    let query2 = match ast.pop().unwrap() {
        Statement::Query(query2) => query2,
        _ => return format!("Not a select query"),
    };
    let select = match &query2.body {
        SetExpr::Select(select) => select,
        _ => return format!("Only select query supported!"),
    };
    let name = get_table_name(select.clone());
    let cols = get_cols(select.projection.clone());
    if let Some(where_eq) = select.selection.clone() {
        let expression = where_fn(where_eq);
        println!("{}", expression);
    }
    let s = "male".to_string();
    println!("{:?}", s.get(0..2).unwrap());
    format!(
        "projection:{:?}\nname:{}\ncols:{:?}\nquery:{:?}\nast:{:?}",
        select, name, cols, query, query2
    )
    .to_string()
}

fn get_table_name(select: Box<Select>) -> String {
    let rel = &select.from[0];
    match &rel.relation {
        TableFactor::Table { name, .. } => match name {
            ObjectName(t) => {
                let table_name = &t[0].value;
                return table_name.to_string();
            }
        },
        _ => return format!("Not implemented!").to_string(),
    }
}

fn get_cols(projection: Vec<SelectItem>) -> Vec<ColInfo> {
    let mut cols = Vec::new();
    let mut col = ColInfo::new();
    let mut fun: Option<String> = None;
    let mut alias: Option<String> = None;
    let mut name: String;
    for item in projection {
        match item {
            SelectItem::UnnamedExpr(item) => {
                match &item {
                    Expr::Function(f) => {
                        fun = Some(f.name.to_string());
                        name = item.to_string();
                        println!("{:?}", f.args[0].to_string());
                        // match f.name.to_string().to_uppercase().as_str() {
                        //     "CONCAT" => {
                        //         let mut cols = Vec::new();
                        //         for c in f.args.iter() {
                        //             cols.push(c.to_string());
                        //         }
                        //         println!("cols : {:?}", cols);
                        //     }
                        // }
                    }
                    _ => {
                        name = item.to_string();
                        fun = None
                    }
                };
                col = ColInfo {
                    name,
                    alias: None,
                    fun: fun,
                };
            }
            SelectItem::ExprWithAlias { expr, alias } => {
                match expr {
                    Expr::Function(f) => {
                        fun = Some(f.name.to_string());
                        name = f.args[0].to_string();
                        match f.name.to_string().to_uppercase().as_str() {
                            "CONCAT" => {
                                let mut cols = Vec::new();
                                for c in f.args.iter() {
                                    cols.push(c.to_string());
                                }
                                println!("cols : {:?}", cols);
                            }
                            _ => todo!(),
                        }
                    }
                    _ => {
                        name = expr.to_string();
                        fun = None;
                    }
                };
                col = ColInfo {
                    name,
                    alias: Some(alias.to_string()),
                    fun,
                };
            }
            SelectItem::Wildcard => {
                col = ColInfo {
                    name: "*".to_string(),
                    alias: None,
                    fun: None,
                }
            }
            _ => {
                col = ColInfo {
                    name: "error".to_string(),
                    alias: None,
                    fun: None,
                }
            }
        };
        cols.push(col);
    }
    cols
}

fn where_fn(expression: Expr) -> String {
    match expression {
        Expr::BinaryOp { left, op, right } => {
            // println!("{}", meval::eval_str(expr).unwrap());
            return format!("left:{}\nop:{}\nright:{}", left, op, right);
        }
        Expr::UnaryOp { op, expr } => format!("left:{}\nop:{:?}", op, expr),
        _ => format!("not a BinaryOp"),
    }
}

// SELECT a.studentid, a.name, b.total_marks FROM student a, marks b;
// SELECT a.studentid, a.name, b.total_marks FROM student a, marks b WHERE a.studentid = b.studentid AND b.total_marks > (SELECT total_marks FROM marks WHERE studentid =  'V002');
// checkout https://www.w3resource.com/sql/subqueries/understanding-sql-subqueries.php

// SELECT CompanyName,  ProductCount = (SELECT COUNT(P.id) FROM [Product] P WHERE P.SupplierId = S.Id) FROM Supplier S;
// SELECT Orders.OrderID, Customers.CustomerName, Orders.OrderDate FROM Orders;
// SELECT * FROM Customers LIMIT 3;

// i like super man!!!!
