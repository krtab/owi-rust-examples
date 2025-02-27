use owi_sym::Symbolic;

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Symbolic for Point {
    fn symbol_inner() -> Self {
        Point { x: Symbolic::symbol_inner(), y: Symbolic::symbol_inner() }
    }
}

fn main() {
    let p = Point::symbol();
    let mid = (p.x + p.y) / 2;
    owi_sym::assert(mid <= p.x.max(p.y));
    owi_sym::assert(p.x.min(p.y) <= mid);
}
