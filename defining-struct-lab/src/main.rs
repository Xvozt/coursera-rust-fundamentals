#[derive(Debug)]
#[allow(dead_code)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: String,
}

impl Person {
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: "john.doe@example.com".to_string(),
        phone_number: "123-456-7890".to_string(),
    };
    println!("{:?}", person);
    println!("{}", person.full_name());
}
