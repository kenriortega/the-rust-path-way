// tipado estatico : chequeo de los tipos de datos se hace en
// tiempo de compilacion
fn main() {
    let num = { 4 };
    show_greeting(select_number(num));

    greet("x".to_string());
    greet2("x");

    // cuando se pasa por referencia no se crea un objeto nuevo
    let r = pass_numb_by_ref(&2);
    show_greeting(r);
}
// la convension es crear los nombres usando snake_case

fn show_greeting(x: i32) {
    println!("H {}", x)
}

fn select_number(x: i32) -> i32 {
    // se puede omitir el return
    return x;
}

// uso  greet("x".to_string())
fn greet(x: String) {
    println!("H {}", x)
}

// uso  greet("x".to_string())
fn greet2(x: &str) {
    println!("H {}", x)
}

fn pass_numb_by_ref(nro: &i32) -> i32 {
    *nro + 4
}
