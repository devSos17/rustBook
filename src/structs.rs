use std::fmt::format;

use rand::random;

#[derive(Debug)]
pub struct User {
    username: String,
    alias: String,
    is_active: bool,
    id: usize,
    email: String,
}

impl User {
    // TODO: OPTIMIZE WITH LIFETIMES (when i learn how to ? )
    pub fn create(username: String, alias: String) -> User {
        User {
            username: username.to_string(),
            alias,
            is_active: true,
            id: random::<usize>(),
            email: gen_email(username), // TODO: OPTION MATCH -> NONE ?
        }
    }

    pub fn recreate_id(&mut self) -> &User {
        self.id = random::<usize>();
        self
    }

    pub fn update_mail(&mut self, email: String) -> &User {
        self.email = email;
        self
    }
}

fn gen_email(username: String) -> String {
    format!("{username}@email.com")
}
