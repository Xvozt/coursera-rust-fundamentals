use std::time::SystemTime;

#[derive(Debug)]
enum Error {
    InvalidEmail,
    InvalidUri,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidEmail => write!(f, "Invalid email"),
            Error::InvalidUri => write!(f, "Invalid URI"),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
    created_at: SystemTime,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
            created_at: SystemTime::now(),
        }
    }

    fn from_email(email: String) -> Result<Self, Error> {
        if email.is_empty() {
            return Err(Error::InvalidEmail);
        };
        if !email.contains("@") {
            return Err(Error::InvalidEmail);
        };

        let username = email.split("@").next().unwrap().to_string();
        let uri = format!("https://{}.com", username);

        Ok(Self {
            username,
            email,
            uri,
            active: true,
            created_at: SystemTime::now(),
        })
    }
    fn deactivate(&mut self) {
        self.active = false;
    }

    fn update_uri(&mut self, new_uri: String) -> Result<(), Error> {
        if new_uri.is_empty() {
            return Err(Error::InvalidUri);
        };
        self.uri = new_uri;
        Ok(())
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("sergeykupriyanov"),
        String::from("skupriyanov@example.com"),
        String::from("https://skupriyanov.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );

    let mut user_from_email = User::from_email(String::from("some_mail@yahoo.com")).unwrap();
    println!("{:?}", user_from_email);

    user_from_email
        .update_uri(String::from("https://new-address.com"))
        .unwrap();
    println!("{:?}", user_from_email);
}
