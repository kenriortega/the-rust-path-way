#[allow(dead_code)] // esto es para q no alert por codigo no usado

// enum OPtion
fn main() {
    let name: Option<String> = Some("Kali".to_string());

    match name {
        None => println!("Name is none"),
        Some(name) => println!("{}", name),
    };

    let new_user = User {
        name: "as".to_string(),
        age: Some(32),
    };

    let age = new_user.get_age();
    match age {
        Some(age) => println!("{}", age),
        _ => (),
    }
}

struct User {
    name: String,
    age: Option<i32>,
}
impl User {
    fn get_age(&self) -> Option<i32> {
        return self.age;
    }
}
