use crate::schema::{accounts,transactions};
use diesel::sql_types::*;
use serde::{Deserialize, Serialize};
use std::default::Default;

//Object structure for  Querying and inserting users
#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "accounts"]
pub struct Account {
    pub id: i64,
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub mobile: String,
    pub accountnmb: String,
    pub opening_balance: i64,
    pub current_balance: i64,
    pub password: String,
    pub ip_address: String,
    pub isactive: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, QueryableByName, Clone)]
#[table_name = "transactions"]
pub struct Transaction {
    pub id: i64,
    pub accountnmb: String,
    pub recepient_accnt_nmb: String,
    pub recepient_name: String,
    pub amount: i64,
    pub trans_type: String,
    pub trans_mode: String,
    pub createdat: Option<chrono::NaiveDateTime>,
    pub updatedat: Option<chrono::NaiveDateTime>,
}

//Structure for Login Response
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub message: Msg,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Msg {
    pub code: String,
    pub description: String,
}

//Methods for Response
impl Response {
    pub fn new() -> Response {
        Response {
            success: false,
            message: Msg {
                code: "".to_string(),
                description: "".to_string(),
            },
            
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Insertable, Default)]
#[table_name = "accounts"]
pub struct NewAccount {
    pub firstname: String,
    pub lastname: String,
    pub username: String,
    pub email: String,
    pub mobile: String,
    pub accountnmb: String,
    pub opening_balance: i64,
    pub password: String,
    pub ip_address: String,
    pub isactive: Option<bool>,
    pub created_at: Option<chrono::NaiveDateTime>,
    pub created_by: Option<String>,
    pub updated_at: Option<chrono::NaiveDateTime>,
    pub updated_by: Option<String>,
}



#[derive(Debug, Serialize, Deserialize, QueryableByName)]
pub struct NUsers {
    #[sql_type = "BigInt"]
    pub id: i64,
    #[sql_type = "Text"]
    pub firstname: String,
    #[sql_type = "Text"]
    pub lastname: String,
    #[sql_type = "Text"]
    pub username: String,
    #[sql_type = "Text"]
    pub email: String,
    #[sql_type = "Text"]
    pub mobile: String,
    #[sql_type = "Text"]
    pub accountnmb: String,
    #[sql_type = "BigInt"]
    pub opening_balance: i64,
    #[sql_type = "BigInt"]
    pub current_balance: i64,
    #[sql_type = "Text"]
    #[serde(skip_serializing)]
    pub password: String,
    #[sql_type = "Text"]
    pub ip_address: String,
    #[sql_type = "Nullable<Bool>"]
    pub isactive: Option<bool>,
    #[sql_type = "Nullable<Timestamp>"]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[sql_type = "Nullable<Text>"]
    pub created_by: Option<String>,
    #[sql_type = "Nullable<Timestamp>"]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[sql_type = "Nullable<Text>"]
    pub updated_by: Option<String>,
}
