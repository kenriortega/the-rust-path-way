// struct

struct User {
    name: String,
    email: String,
    age: i32,
    status: bool,
}

impl User {
    fn age(&self) -> i32 {
        return self.age;
    }
    fn email(&self) -> String {
        return self.email.to_string();
    }
}
fn main() {
    let mut user = User {
        name: "k".to_string(),
        email: String::from("k@mail.com"),
        age: 30,
        status: true,
    };
    user.status = false;
    println!("User {}", user.name);
    let user2 = new_user(String::from("jo"), String::from("jo@mail.com"));
    println!("User2 {}", user2.age);

    let user1 = User {
        name: "kas".to_string(),
        email: "as@mai.com".to_string(),
        ..user
    };
    println!("User {}", user1.name);

    // tuples struct
    struct Point(i32, i32, i32);
    let p = Point(23, 3, 4);
}

fn new_user(name: String, email: String) -> User {
    return User {
        name,
        email,
        age: 30,
        status: true,
    };
}
