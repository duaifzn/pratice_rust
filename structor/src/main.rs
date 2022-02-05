pub mod user;
fn main() {
    let user1 = user::User::build_user("duai", "duaifzn@gmail.com");
    
    dbg!(user1);
}