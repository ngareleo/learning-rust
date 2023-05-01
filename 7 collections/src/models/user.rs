
#[derive(Debug)]
pub struct User {
    pub username: String,
    password: String,
}

impl User {
    pub fn from(username: String, password: String) -> User {
        User {
            username,
            password,
        }
    }        
}