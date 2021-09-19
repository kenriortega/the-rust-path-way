use std::{ops::Deref, rc::Rc};

#[allow(dead_code)] // esto es para q no alert por codigo no usado

fn main() {
    // referencias (&)
    // let x = 5;
    // let y = &x;
    // Reference counter (contador de referencias)
    // String y el Vec<T>

    // Los smart pointers son usualmente implementados usando structs,
    // pero implementando los traits Deref y Drop
    // Deref permite a las instacias de Smart pointers comportarse como referencias
    // para que el mismo codigo q funciona con referencias, funcione con smart pointers

    // Drop trait permite definir logica que se ejecute una vez que
    // el smart pointer sale del scope

    // Box<T = caja
    // sirve para definir cuanta memoria necesita en el stack
    // estando en el heap
    // heap
    // some_box()

    // Reference Counted Smart Pointer: permite que un valor tenga muchos dueños
    // Usamos Rc cuando queremos asignar datos en el heap para que sea accedido
    // en multiples partes del código, y no podemos determinar en tiempo de
    // compilación el último que accederá estos datos. Si supieramos de antemano
    //  quien sería el último, podríamos hacer que ese último sea el dueño,
    // pero no lo sabemos. Entonces Rc lleva un contador de referencias con
    // todos los dueños, y cuando ya no quedan más dueños, puede limpiar el dato.
}

fn some_defer() {
    // hace posible la deferenciacion (*)
    let x = 5;
    let y = &x;
    let z = &y;

    if x == 5 {
        println!("{}", 5);
    }

    if *y == 5 {
        println!("{}", 5);
    }

    if **z == 5 {
        println!("{}", 5);
    }

    let mb = MyBox::new(x);
    if *mb == 5 {
        println!("{}", 5);
    }
}

struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Drop")
    }
}
fn some_box() {
    let x = 12;
    let y = Box::new(2);
    println!("{}", y);

    // linked list
    // (v,nodo1) -> (v,nodo2)

    enum List {
        Node(i32, Rc<List>),
        None,
    }
    let node3 = List::Node(100, Rc::new(List::None));
    let node2 = List::Node(150, Rc::new(node3));
    let node2_rc = Rc::new(node2);
    let node1 = List::Node(10, Rc::clone(&node2_rc)); // deep clone
    let node0 = List::Node(130, Rc::clone(&node2_rc));
}
