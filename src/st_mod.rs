struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn new(email: String, username: String) -> User {
        User {
            email: email,
            username: username,
            active: true,
            sign_in_count: 1,
        }
    }

    fn info(&self) -> String {
        // self.email.to_string()
        //     + &"-"
        //     + &self.username
        //     + &"-"
        //     + &self.sign_in_count.to_string()
        //     + &"-"
        //     + &self.active.to_string()

        format!(
            "{}-{}-{}-{}",
            self.email, self.username, self.sign_in_count, self.active
        )
    }
}

// fn info(user: &User) -> String {
//     user.email.to_string()
//         + &"-"
//         + &user.username
//         + &"-"
//         + &user.sign_in_count.to_string()
//         + &"-"
//         + &user.active.to_string()
// }

pub fn build(email: String, username: String) {
    // let user1 = User {
    //     email,
    //     username,
    //     active: true,
    //     sign_in_count: 1,
    // };
    let user1 = User::new(email, username);
    // println!("{}", info(&user1));
    println!("{}", user1.info())
}
