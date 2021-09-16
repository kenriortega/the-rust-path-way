#[allow(dead_code)] // esto es para q no alert por codigo no usado

fn main() {
    // closure: funcion que es definida en linea (inline)
    // let sum = |nro: i32| -> i32 { nro + 1 };

    // // no inline
    // // let sum = sum_one;
    // println!("{}", sum(2));

    let mut count = 1;
    let mut inc = move || count += 1;

    let variable = &count; // borrowing, pedir prestada
    inc();
    println!("{}", count)
}

fn sum_one(nro: i32) -> i32 {
    nro + 1
}
