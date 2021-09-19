use std::fs::File;
use std::io::ErrorKind;
#[allow(dead_code)] // esto es para q no alert por codigo no usado

fn main() {
    // manejo de errores
    // Recuperables e.g: abrir archivos donde el path es incorrecto
    // no hay exceptions
    // Result<T,E>
    let file = File::open("./main.rs");
    match file {
        Ok(file) => read_file(file),
        Err(error) => match error.kind() {
            ErrorKind::NotFound => println!("NotFound"),
            other_error => println!("Other error"),
        },
    }

    let fi2 = File::open("./main.rs").unwrap();
    let fi2 = File::open("./main.rs").expect("NotFound");
    // No-Recuperables e.g: tratar de acceder a un arreglo mas alla de su limite
    // tiene un macro llamado panic!()
    // panic!("explota")
}

fn read_file(file: File) {}
