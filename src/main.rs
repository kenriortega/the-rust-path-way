#[allow(dead_code)]

// Macros
// codigo que escribe codigo (meta-programing)

// macro declarativo

macro_rules! tres {
    ($x:expr) => {
        $x + 1
    };
}

macro_rules! my_vec {
    ($($x:expr),*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.push($x);
            )*
            vec
        }
    };
}
macro_rules! func {
    ($name:ident) => {
        fn $name() {
            println!("hola {:?}", stringify!($name))
        }
    };
}
fn main() {
    println!("{}", tres!(2));
    let vector = my_vec![2, 23];
    println!("{:?}", vector);
    func!(greet);
    greet();
}
