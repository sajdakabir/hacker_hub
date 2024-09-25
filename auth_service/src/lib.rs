#![allow(dead_code,unused_variables)]

mod database;

pub mod auth_utils;

use database::{Status, connect_to_databse};


pub fn authenticate(cred: auth_utils::models::Credentials) {

   if let Status::Conncted=connect_to_databse(){
    auth_utils::login(cred);
   }

}