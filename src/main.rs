#[allow(dead_code)] // esto es para q no alert por codigo no usado

fn main() {
    let s = [1, 2, 3];

    for x in s.iter() {
        println!("{}", x)
    }

    let mut c = Counter::new();
    let i = c.next();
    match i {
        Some(count) => println!("{:?}", count),
        _ => (),
    }
}

struct Counter {
    count: i32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}
