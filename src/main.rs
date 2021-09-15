#[allow(dead_code)] // esto es para q no alert por codigo no usado

// traits == rasgo en espannol
fn main() {
    let k = Humman;
    k.say_hello();
    let pelusa = Cat;
    pelusa.say_hello();

    println!("{}", Humman::run());

    let age = Some(3);
    println!("{}", age.is_gt_age());

    // macros son funciones de rust q escriben codigo rust
    // macro format y println
    let u = User {
        name: "L".to_string(),
        age: 18,
    };

    println!("{}", u);
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{} ({})", self.name, self.age)
    }
}
// impl std::fmt::Debug for User {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("User")
//             .field("name", &self.name)
//             .field("age", &self.age)
//             .finish()
//     }
// }
// #[derive(Debug)]
struct User {
    name: String,
    age: i32,
}
struct Humman;
struct Cat;

trait Speak {
    fn say_hello(&self) -> String;
    // al no recivir slef como parametro se considera static
    // por lo que no es obligatorio impl
    fn run() -> String {
        "run".to_string()
    }
}

impl Speak for Humman {
    fn say_hello(&self) -> String {
        " todo!()".to_string()
    }
}

impl Speak for Cat {
    fn say_hello(&self) -> String {
        "miau".to_string()
    }
}

trait DriverLicence {
    fn is_gt_age(&self) -> bool;
}

impl DriverLicence for Option<i32> {
    fn is_gt_age(&self) -> bool {
        match self {
            Some(age) => age > &18,
            None => false,
        }
    }
}
