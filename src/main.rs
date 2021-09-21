#[allow(dead_code)]
// esto es para q no alert por codigo no usado
// Concurrency = Concurrencia
// Programacion concurrente con rust
// Paralelismo
// Comunicacion entre threads channels
// Mutexes: permiten el acceso al mismo dato d
// desde diff threard , por turnos
use std::thread;
use std::{
    rc::Rc,
    sync::{mpsc, Arc, Mutex},
    time::Duration,
}; // multiple producer single consumer
fn main() {
    // some_thread()
    // some_txrx();

    // Mutex("hola")
    // lock = bloqueador
    // lock("hola")
    // liberar lock

    // Arc es thread safe es seguro para usarlo en situaciones concurrentes
    // Atomic reference counter
    // atomic son primitivos seguros de compartir en diferentes threads
    let id = Arc::new(Mutex::new(0));
    // {
    //     let mut num = id.lock().unwrap();
    //     *num = 88;
    // }
    let mut handles = vec![];
    for _ in 0..3 {
        let id_arc = Arc::clone(&id);
        let h = thread::spawn(move || {
            let mut num = id_arc.lock().unwrap();
            *num += 1;
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
    println!("{:?}", id);
}

fn some_txrx() {
    let name = String::from("kalix");
    println!("{}", name);
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();
    thread::spawn(move || {
        println!("th1 {}", name);

        for count in 0..3 {
            // send data to main
            let mut message = String::from("counter: ");
            message.push(char::from_digit(count, 10).unwrap());
            tx.send(message).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });
    thread::spawn(move || {
        for count in ['a', 'e', 'i', 'o', 'u'].iter() {
            // send data to main
            let mut message = String::from("letras: ");
            message.push(*count);
            tx2.send(message).unwrap();
            thread::sleep(Duration::from_millis(2000));
        }
    });
    for msg in rx {
        println!("{}", msg);
    }
}

fn some_thread() {
    let name = String::from("kalix");
    let name2 = name.clone();
    println!("Hola {}", name);

    // manejador de join
    let join_handle = thread::spawn(move || {
        println!("Thread 1 {}", name2);
        thread::sleep(Duration::from_millis(2000));
    });

    join_handle.join().unwrap();
    println!("Ambos thread are finished {}", name)
}
