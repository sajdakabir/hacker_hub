use auth_service::authenticate;
use auth_service::auth_utils::models::Credentials;


fn main() {
    let cre= Credentials{
        username: "sajda".to_owned(),
        password:"ehej".to_owned()
    };

    authenticate(cre);
}
