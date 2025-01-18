use std::rc::Rc;

struct Gene {}
pub struct Wyrm {
    x: i32,
    y: i32,
    dir: (i32, i32),
    age: i32,
    genome: Vec<Gene>,
}
