// Ignore unused stuff for now
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use std::{env, collections::HashMap, io::{stdin, stdout, Write}};
use dotenv::dotenv;
use firebase_rs::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct User {
    name: String,
    age: i32,
    email: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Response {
    name: String,
}

#[tokio::main]
async fn main() {

    dotenv().ok();

    let firebase_uri = env::var("FIREBASE_REALTIME_DB_URI").expect("Missing database URI");
    let firebase_auth_key = env::var("FIREBASE_AUTH_KEY").expect("Missing Firebase auth key.");

    // let firebase = Firebase::new(firebase_uri).unwrap();
    let firebase = Firebase::auth(&firebase_uri, &firebase_auth_key).unwrap();

    let mut name_input = String::new();
    let mut age_input = String::new();
    let mut email_input = String::new();

    print!("Name: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut name_input)
        .expect("Failed to read user input");

    print!("Age: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut age_input)
        .expect("Failed to read user input");

    print!("Email: ");
    stdout().flush().unwrap();

    stdin()
        .read_line(&mut email_input)
        .expect("Failed to read user input");

    let age_input_num: i32 = age_input.trim().parse().expect("Input is not a number");

    println!();
    
    let user = User {
        name: name_input.trim().to_string(),
        age: age_input_num,
        email: email_input.trim().to_string(),
    };

    let response = set_user(&firebase, &user).await;
    
    let mut user = get_user(&firebase, &response.name).await;
    println!("\nUser: \n{:?}", user);

    let users = get_users(&firebase).await;
    println!("\nAll users: \n{:?}", users);

    // user.email = "new.email@gmail.com".to_string();
    // let updated_user = update_user(&firebase, &response.name, &user).await;
    // println!("\nUpdated user: \n{:?}", updated_user);

    // delete_user(&firebase, &response.name).await;

    // println!("\nUser deleted.\n");
    // println!("\nAll users: \n{:?}", users);
}

async fn set_user(firebase_client: &Firebase, user: &User) -> Response {
    let firebase = firebase_client.at("users");
    let users = firebase.set::<User>(&user).await;

    return string_to_response(&users.unwrap().data);
}

async fn get_users(firebase_client: &Firebase) -> HashMap<String, User> {
    let firebase = firebase_client.at("users");
    let users = firebase.get::<HashMap<String, User>>().await;

    return users.unwrap();
}

async fn get_user(firebase_client: &Firebase, id: &String) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let user = firebase.get::<User>().await;

    return user.unwrap();
}

async fn update_user(firebase_client: &Firebase, id: &String, user: &User) -> User {
    let firebase = firebase_client.at("users").at(&id);
    let _user = firebase.update::<User>(&user).await;

    return string_to_user(&_user.unwrap().data);
}

async fn delete_user(firebase_client: &Firebase, id: &String){
    let firebase = firebase_client.at("users").at(&id);
    let _result = firebase.delete().await;
}

fn string_to_response(s: &str) -> Response {
    serde_json::from_str(s).unwrap()
}

fn string_to_user(s: &str) -> User {
    serde_json::from_str(s).unwrap()
}