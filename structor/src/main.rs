use user::User;
pub mod user;
fn main() {
    let user = User::build_user("duai", "duaifzn@gmail.com");
    
    dbg!(user);
}