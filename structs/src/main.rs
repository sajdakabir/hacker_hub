
struct User {
    username: String,
    email: String,
    active:bool,
    sing_in_ount: u64
}
fn main() {

    let mut user_1 = build_user(String::from("saju"), String::from("gmail"));
    println!("the value of userName {}", user_1.username);
    user_1.username.push_str(" kabir");
    println!("the value of userName {}", user_1.username);

}

fn build_user(username: String, email: String)-> User {
    User {
    username,
    email,
    active: true,
    sing_in_ount:1
   }
}
