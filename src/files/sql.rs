extern crate meval;
extern crate sqlparser;
use sqlparser::{
    ast::{Expr, ObjectName, Select, SelectItem, SetExpr, Statement, TableFactor},
    dialect::GenericDialect,
    parser::Parser,
};

#[get("/sql/select")]
pub fn sql_select() -> String {
    let dialect = GenericDialect {};
    let query = "SELECT AVG(col) FROM table_1 where -30+40";
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

fn get_cols(projection: Vec<SelectItem>) -> Vec<String> {
    let mut cols = Vec::new();
    for item in projection {
        let col = match item {
            SelectItem::UnnamedExpr(item) => item.to_string(),
            SelectItem::Wildcard => return vec![format!("select all columns from table")],
            _ => return vec![format!("not a UnnamedExpr!")],
        };
        cols.push(col);
    }
    cols
}

fn where_fn(expression: Expr) -> String {
    match expression {
        Expr::BinaryOp { left, op, right } => {
            let expr = format!("{}{}{}", left, op, right);
            println!("{}", meval::eval_str(expr).unwrap());
            return format!("left:{}\nop:{}\nright:{}", left, op, right);
        }
        Expr::UnaryOp { op, expr } => format!("left:{}\nop:{:?}", op, expr),
        _ => format!("not a BinaryOp"),
    }
}
