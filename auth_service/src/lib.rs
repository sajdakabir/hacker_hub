#![allow(dead_code,unused_variables)]

mod database {
   pub enum Status {
        Conncted,
        NotConnected   
    }
    
   pub fn connect_to_databse()-> Status{
        Status::Conncted
    }
    
   pub fn get_user(){
    
    }
    
}

pub mod auth_utils {
    pub fn login(cred:models::Credentials){
        //fetch the database and get the user
        // crate::database:: get_user();
        super::database:: get_user();
    }

   pub mod models {
        pub struct Credentials {
            pub username: String,
            pub password: String
        }
        
    }
    
}



pub fn authenticate(cred: auth_utils::models::Credentials) {

   if let database::Status::Conncted=database::connect_to_databse(){
    auth_utils::login(cred);
   }

}