#[allow(dead_code)] // esto es para q no alert por codigo no usado

// generics
fn main() {
    let pt1 = Point { x: 0, y: 12 };
    let pt2 = Point { x: 0, y: 12 };
    calc_area(pt1, pt2)
}

struct Point<T> {
    x: T,
    y: T,
}

fn calc_area<T>(pt1: Point<T>, pt2: Point<T>) {}
