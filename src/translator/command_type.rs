use nom_sql::{CreateTableStatement, InsertStatement};

pub enum CommandType {
    GetSchema(String),
    SetSchema(CreateTableStatement),
    SetData(InsertStatement),
    GetData(String),
}
