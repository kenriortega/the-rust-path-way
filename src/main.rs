use std::collections::{HashMap, HashSet};

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
    // some_vector();
    // strings
    // some_string();
    // hashmap
    // some_hashmap();
    //hashset
    some_hashset()
}

fn some_hashset() {
    // no permite duplicados

    let mut user_id: HashSet<i32> = HashSet::new();
    user_id.insert(100);
    user_id.insert(900);
    user_id.insert(2);

    // for id in user_id.iter() {
    //     println!("{}", id)
    // }

    // la api ofrece los siguientes methods
    // union obtener los elementos unicos entre 2 sets
    // difference: obtener los elementos q estan en el primer set y no en el otro
    // intersection: obtener solo los elementos q estan en ambos sets
    let mut backup_users = HashSet::new();
    backup_users.insert(100);
    backup_users.insert(23);
    backup_users.insert(9);

    for id in user_id.difference(&backup_users) {
        println!("diff {}", id)
    }
    for id in user_id.union(&backup_users) {
        println!("union {}", id)
    }
    for id in user_id.intersection(&backup_users) {
        println!("inter {}", id)
    }
}
fn some_hashmap() {
    // structura de datos que permite guardar valores
    // key deben ser del mismo tipo de dato

    let mut kv: HashMap<String, i32> = HashMap::new();
    kv.insert(String::from("server"), 1);
    kv.insert(String::from("port"), 1);

    let serv = kv.get("server"); // ownership, borrowing

    for (k, v) in &kv {
        println!("k: {}, v: {}", k, v);
    }

    match serv {
        Some(v) => println!("ok {}", v),
        None => (),
    }
}

fn some_string() {
    // existe dos formas string y slice (referencia continua de elementos de una collection)
    // son de tipo u8
    // string se guarda en el heap
    // string slice se guarda en el stack porq es una referencia al heap ya q estan incrustado en el codigo binario
    let mut hello = String::from("ks");
    let nom = "ja"; // literal or hardcode se alamcena en el binario
    let mut nom2 = nom.to_string();
    nom2.push('s');

    let nom3 = &hello[..hello.len()];
    hello.push('a');
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
