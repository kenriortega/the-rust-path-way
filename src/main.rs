// enums

struct User {
    name: String,
    email: String,
    age: i32,
    status: bool,
    user_role: UserRole,
    website: Website,
}

enum UserRole {
    BASIC,
    ADMIN,
}

enum Website {
    URL(String),
    INSTAGRAM(String),
    LINKEDIN(String),
    FACEBOOK(String),
}

fn main() {
    let mut user = User {
        name: "k".to_string(),
        email: String::from("k@mail.com"),
        age: 30,
        status: true,
        user_role: UserRole::BASIC,
        website: Website::INSTAGRAM("Instagram".to_string()),
    };
    user.status = false;
    println!("User {}", user.name);

    let access = hasAccess(user.user_role);
}

fn hasAccess(user_role: UserRole) -> bool {
    match user_role {
        UserRole::ADMIN => true,
        UserRole::BASIC => false,
    }
}
