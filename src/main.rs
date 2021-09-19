#[allow(dead_code)] // esto es para q no alert por codigo no usado

fn main() {
    // Test unitarios
    // assert!(expression)
    // assert_eq!(a,b)
    // assert_ne!(a,b)
}

fn divide(a: i32, b: i32) -> i32 {
    if b == 0 {
        panic!("No puedes dividir por 0")
    }
    a / b
}

fn sum(a: i32, b: i32) -> i32 {
    a + b
}

fn check_alphanumeric_code(code: &str) -> bool {
    code.chars().all(char::is_alphanumeric)
}

#[test]
fn check_alphanumeric_code_test() {
    let r = check_alphanumeric_code("12356");
    assert!(r)
}
#[test]
fn check_alphabetic_code_test() {
    let r = check_alphanumeric_code("L12356");
    assert!(r)
}

#[test]
#[ignore]
fn sum_test() {
    assert_eq!(sum(2, 2), 4);
}

#[test]
#[should_panic]
fn divide_test() {
    divide(12, 0);
}
