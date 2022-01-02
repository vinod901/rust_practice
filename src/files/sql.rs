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

#[get("/sql/select")]
pub fn sql_select() -> String {
    let dialect = GenericDialect {};
    let query = "SELECT left(col_2,2) FROM table_1 WHERE col_1 > col_2";
    let mut ast = Parser::parse_sql(&dialect, query).unwrap();
    let query2 = match ast.pop().unwrap() {
        Statement::Query(query2) => query2,
        _ => return format!("Not a select query"),
    };
    let select = match query2.body {
        SetExpr::Select(select) => select,
        _ => return format!("Only select query supported!"),
    };
    let name = get_table_name(select.clone());
    let cols = get_cols(select.projection.clone());
    if let Some(where_eq) = select.selection.clone() {
        let expression = where_fn(where_eq);
        println!("{}", expression);
    }
    format!("projection:{:?}\nname:{}\ncols:{:?}", select, name, cols).to_string()
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
                        println!("function : {}", f.name.to_string());
                        fun = Some(f.name.to_string());
                        name = item.to_string();
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
