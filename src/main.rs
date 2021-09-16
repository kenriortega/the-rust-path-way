// Flujos de control

#[allow(dead_code)] // esto es para q no alert por codigo no usado

fn main() {
    // if
    let number = 5;
    if number == 5 {
        println!("es {}", number);
    } else {
        println!("No es 5")
    }

    let result = if number >= 5 { 1000 } else { 12 };
    println!("{}", result);

    let mut counter = 0;
    // loops
    let r = loop {
        println!("loop");
        if counter == 10 {
            break counter;
        }
        counter += 1;
    };
    println!("{}", r);

    while counter > 0 {
        println!("{}", counter);
        counter -= 1;
    }

    let arreglo = [1, 2, 3, 4];
    for element in arreglo.iter() {
        println!("{}", element)
    }
    for e in 0..4 {
        println!("{}", e);
    }

    let age: Option<i32> = Some(23);
    match age {
        Some(value) => println!("{}", value),
        _ => (),
    };
    if let Some(value) = age {
        println!("{}", value);
    };

    let mut message_not_checked = Some(100);
    loop {
        match message_not_checked {
            Some(value) => {
                if value > 0 {
                    println!("message");
                } else {
                    println!("not message");
                    message_not_checked = None;
                }
            }
            None => {
                break;
            }
        }
    }

    while let Some(value) = message_not_checked {
        if value > 0 {
            println!("message");
        } else {
            println!("not message");
            message_not_checked = None;
        }
    }
}
