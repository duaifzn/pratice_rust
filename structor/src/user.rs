#[derive(Debug)]
pub struct User{
    name: String,
    email: String,
    active: bool,
    sign_in_count: u32,
}

impl User{
    pub fn build_user(name: &str, email: &str) -> User{
        return User{
            name: name.to_string(),
            email: email.to_string(),
            active: true,
            sign_in_count: 1,
        }
    }
}
