use crate::model::structs;
use crate::vars;
extern crate bcrypt;


use bcrypt::{hash};
use uuid::Uuid;
use diesel::PgConnection;
use diesel::{prelude::*, sql_query};
use dotenv::dotenv;

//TODO; implement r2d2 for connection pool
// Error handling
//Create new database connection and return
pub fn getdbconn() -> PgConnection {
    dotenv().ok();
    let database_url = vars::db_url();
    PgConnection::establish(&database_url).unwrap()
}

//Query database for username
pub fn fetch_user(jusername: &str) -> std::vec::Vec<structs::Account> {
    use crate::schema::accounts::dsl::*;

    //create new databse connection
    let connection = getdbconn();

    accounts
        .filter(username.eq(jusername))
        .load::<structs::Account>(&connection)
        .unwrap()
}


pub fn get_balance(acc: &str)-> std::vec::Vec<structs::Account> {
     
    use crate::schema::accounts::dsl::*;

    //create new databse connection
    let connection = getdbconn();

    accounts
        .filter(accountnmb.eq(acc))
        .load::<structs::Account>(&connection)
        .unwrap()
}


pub fn create_user(mut jsondata: structs::NewAccount) -> Result<bool, actix_web::error::Error> {
    use crate::schema::accounts::dsl::*;
    let connection = getdbconn();
    let my_uuid = Uuid::new_v4();
    // let mut jsondata = jsondata;
    let encrypted_password = hash_pass(&jsondata.password);
    jsondata.password = encrypted_password;
    let  new_acc=structs::Account{
    id:1,
    firstname: jsondata.firstname,
    lastname: jsondata.lastname,
    username: jsondata.username,
    email: jsondata.email,
    mobile: jsondata.mobile,
    accountnmb: my_uuid.to_string(),
    opening_balance: jsondata.opening_balance,
    current_balance: jsondata.opening_balance,
    password: jsondata.password,
    ip_address: jsondata.ip_address,
    isactive:   None,
    created_at: None,
    created_by: None,
    updated_at: None,
    updated_by: None,
        };

    let results = diesel::insert_into(accounts)
        .values(&new_acc)
        .get_results::<structs::Account>(&connection);
    Ok(results.is_ok())
}

pub fn hash_pass(pass_string: &str) -> String {
    // hash
    hash(pass_string,12).unwrap()
}


pub fn list_users() -> Vec<structs::NUsers> {
    let conn = getdbconn();

    // results
    sql_query("SELECT * FROM accounts".to_string())
        .load::<structs::NUsers>(&conn)
        .unwrap()
}
