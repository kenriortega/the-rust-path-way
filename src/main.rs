// las constantes pueden ser globales
const N_SPACES: i32 = 23;
// tipado estatico : chequeo de los tipos de datos se hace en
// tiempo de compilacion
fn main() {
    // las variables son por defecto inmutables
    // se debe agregar el keyword mut para cambiar su inmutabilidad
    let mut x = 12;
    println!("Hello, world! {}", x);
    x = 2 + 3;
    println!("Hello, world! {}", x);

    // shadown cuando se usa el mismo nombre de una variable ya creada
    let spaces = " ";
    let spaces = spaces.len();
    println!("Hello, world! {}", spaces);
    println!("Hello, world! {}", N_SPACES);
}
