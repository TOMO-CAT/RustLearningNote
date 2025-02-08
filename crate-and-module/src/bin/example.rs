use crate_and_module::*;

// cargo run
// cargo run --bin example
fn main() {
    let s = math::add(1, 2);
    println!("1 + 2 = {}", s);

    let r = geometry::shapes::circle_area(10.0);
    println!("circle area = {}", r);
}
