#![allow(dead_code,unused_variables)]
 pub struct Credentials {
    pub username: String,
    pub password: String
}

enum Status {
    Conncted,
    NotConnected   
}

fn connect_to_databse()-> Status{
    Status::Conncted
}

fn get_user(){

}

fn login(cred: Credentials){
    //fetch the database and get the user
    get_user();
}


pub fn authenticate(cred: Credentials) {

   if let Status::Conncted= connect_to_databse(){}

}