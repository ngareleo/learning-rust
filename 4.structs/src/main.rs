#[derive(Debug)]
struct User {
    active: bool,
    email: String,
    sign_in_count: i64,
    username: String,
}

impl User {
    fn create_user(username: String, email: String) -> Self {
        User{
            username,
            email,
            active: true,
            sign_in_count: 1,
        }
    }

    fn create_user_without_email(username: String) -> Self {
        User{
            active: true,
            sign_in_count: 1,
            username,
            email: String::new()
        }
    }

    fn update_email(&mut self, email: String){
        self.email = email;
    }
}

struct Color(u8, u8, u8);

fn main() {
    let mut user = User{
        active: false,
        username: String::from("ngareleo"),
        email: String::from("ngareoel@gmail.com"),
        sign_in_count: 1,
    };
    user.email = String::from("ngarimwenda@gmail.com");
    let username = String::from("tty");
    let email = String::from("tty@gmail.com");
    let new_user = create_user(username, email);
    println!("{}", new_user.username);
    let mut new_user = update_username(new_user, String::from("ttty")); 
    println!("{}", new_user.username);
    println!("{}", new_user.sign_in_count);
    println!("{:?}", new_user);
    user_is_active(&new_user);


    // Tuple structs
    let _black = Color(0, 0, 0);
    new_user.update_email(String::from("email@gmail.com"));
    println!("{:?}", new_user);

    let email_less_user = User::create_user_without_email(String::from("stan"));
    println!("Email less user: {:?}", email_less_user);

    let regular_user = User::create_user(String::from("chucknorris"), String::from("chucknorris@gmail.com"));
    println!("Normal user: {:?}", regular_user);


}


fn create_user(username: String, email: String) -> User {
    User{
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn update_username(user: User, username: String) -> User {
    User{
        username,
        ..user // unpacks the rest
    }
}

fn user_is_active(user: &User) {
    if user.active {
        return println!("User is active")
    }
    println!("User is inactive")
}