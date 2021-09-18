#[allow(dead_code)] // esto es para q no alert por codigo no usado

const VALUE: i32 = 22;
fn main() {
    // lifetime = tiempo de vida de la mem
    // es una forma de asegurar que un pedazo
    // de memoria es aun valida para una referencia

    do_stuff(&VALUE);
    let a;
    {
        // life time B
        let b = 10;
        // a = &b; // cambiando el ownership de b -> a
        a = b; // cambiando el ownership de b -> a
    } // b es liberada
      // println!("{}", b); fuera del scope
    println!("{}", a);
    // let b = get_ref(&a);
    let b = get_ref(&a, &VALUE);
    // let hola = get_owner(String::from("dk"));
    let name: &'static str = "df";
}
// fn get_owner<'a>(param: String) -> &'a str {
//     &param // no compila
// }
fn do_stuff<'a>(param: &'a i32) -> &'a i32 {
    param
}
fn get_ref<'a, 'b>(param: &'a i32, m: &'a i32) -> &'a i32 {
    if param > m {
        param
    } else {
        m
    }
}
