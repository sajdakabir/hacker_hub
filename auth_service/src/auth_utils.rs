pub mod models;

pub fn login(cred:models::Credentials){
    //fetch the database and get the user
    // crate::database:: get_user();
    super::database:: get_user();
}
