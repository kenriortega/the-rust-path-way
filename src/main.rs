// tipado estatico : chequeo de los tipos de datos se hace en
// tiempo de compilacion
fn main() {
    // Integer
    let entero: i8 = 23;
    let entero: u8 = 49;
    let entero2: i8 = -4;

    // interger literals
    let decimal = 90_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;

    // floats point
    let f1 = 5.0;
    let f2: f32 = 12.432;

    // boolean
    let v = true;
    let f = false;

    // character
    let ch = 'a'; // emoji

    // compound types

    // tuplas
    let mitupla = ('h', 23, -3, 0.3333);
    let tu2: (char, i64, u32) = ('h', 34, 1);
    let (x, y, z) = tu2;

    // array tamanno fijo
    let arreglo = [1, 2, 3, 4, 5];

    // strings existen dos tipo
    // string slide
    // es estatico porq es almacenado en el binario finalmente generado
    let nombre: &'static str = "cuba"; // mas primitivo

    // se aloja en la memoria heap
    let appellido: String = "jiji".to_string();
    let appl = String::new();
}
