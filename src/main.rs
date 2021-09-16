// Collections

#[allow(dead_code)] // esto es para q no alert por codigo no usado

fn main() {
    // collections pueden tener multiples valores, y apuntan a la memoria heap
    // por lo q el tamanno no debe ser conocido en tiempo de compilacion

    // vector
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3, 4, 5];

    // v.push(2);
    // v.push(20);
    // v.push(200);
    some_vector();
    // strings
    // hashmap
}

fn some_vector() {
    let mut v = vec![23, 34, 4];
    v.push(3);
    let ter = v.get(2);
    if ter.is_some() {
        println!("{}", ter.unwrap())
    }
    match ter {
        Some(value) => println!("{}", value),
        None => (),
    }

    match v.get(200) {
        Some(value) => println!("{}", value),
        None => (),
    }

    for i in &mut v {
        *i += 10;
    }
    for i in &v {
        println!("{}", i)
    }

    enum Message {
        TEXT(String),
        ERROR(i32),
    }

    let message = vec![Message::TEXT("as".to_string()), Message::ERROR(23)];
    for m in &message {
        match m {
            Message::TEXT(text) => println!("{}", text),
            Message::ERROR(code) => println!("{}", code),
        }
    }
}
