// Ownership = propiedad, que es duenno de .
// Borrowing = pedir prestado.
// Estos concepto hacen referencia al manejo de memoria
// en rust no hay GC
// cada data tiene un owner
#[allow(dead_code)] // esto es para q no alert por codigo no usado

fn main() {
    // Stack mem implementada como una pila
    // rapida para insertar, eliminar y recoger datos
    // es de tamanno fijo
    // es liberada cuando se alcanza el fin del scope (ambito)

    let mut age: i32 = 20; // crear var mut
    increase_age(&mut age); // se pasa por ref la variable y debe ser mutable la ref
    println!("{}", age);

    // Heap mem
    // es flexible, costosos los procesos de asignacion y recuperacion
    // es liberada cuando no tiene mas duennos (owners)
    let mut name = String::from("Julio");
    // send_name(name.clone()); // clone es muy costoso
    send_name(&mut name);
    println!("{}", name);
}

fn increase_age(age: &mut i32) {
    // el metodo espera una ref mut
    *age += 1; // al ser una ref debemos acceder directo a la memoria para desasignarla
}

fn send_name(name: &mut String) {
    name.push('x')
}
