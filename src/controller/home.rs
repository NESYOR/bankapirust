use crate::model::{dbmethods, structs};
use actix_web::{web, HttpResponse};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Info {
    accountnmb: String,
}

pub async fn register_user(jsondata: web::Json<structs::NewAccount>) -> HttpResponse {
    //Declare Body of Response
    let mut rbody = structs::Response::new();

    println!("hit register_user");
    //Convert JSON Request type from "web::Json<structs::Year>" to "Year"
    let new_user = jsondata.into_inner();

    println!("{:?}",new_user);

    //fetch user details from database
    let temp_user = dbmethods::fetch_user(&new_user.username);

    //check if results are empty(user already exists in database), if yes then set response as user already exists
    if !temp_user.is_empty() {
        rbody.message.description = "User already exists".to_string();
        return HttpResponse::Ok().json(&rbody);
    }

    //if not create new user
    let result = dbmethods::create_user(new_user);

    //check whether user created or not and respond
    if result.is_ok() {
        rbody.success = false;
        rbody.message.description = "User created successfully".to_string();
        return HttpResponse::Ok().json(&rbody);
    } else {
        rbody.message.description = "User creation failed".to_string();
        return HttpResponse::Ok().json(&rbody);
    }
}


pub async fn fetch_balance(info:web::Query<Info>) -> HttpResponse {
    
    let mut rbody = structs::Response::new();

    println!("{}",info.accountnmb);
    let temp_user = dbmethods::get_balance(&info.accountnmb);
    
    if temp_user.is_empty() {
        println!("{:?}",temp_user);
        rbody.message.description = "Account doesn't exists".to_string();
        return HttpResponse::Ok().json(&rbody);
    }

    else{
        println!("{:?}",temp_user);
        rbody.message.description = temp_user[0].current_balance.to_string();
        return HttpResponse::Ok().json(&rbody);
    }
}

pub async fn list_users() -> HttpResponse {
    let results = dbmethods::list_users();

    HttpResponse::Ok().json(results)
}