use std::fs;

#[test]
fn test() {
    run();
}

pub fn run() {
    let ex = fs::read_to_string("examples/ex1").expect("Unable to read input!");
}