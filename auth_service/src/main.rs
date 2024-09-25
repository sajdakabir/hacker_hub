use auth_service::{authenticate, Credentials};


fn main() {
    let cre= Credentials{
        username: "sajda".to_owned(),
        password:"ehej".to_owned()
    };

    authenticate(cre);
}
