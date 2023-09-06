#[allow(dead_code)]
#[derive(Clone, Copy, Default)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct Polyline(Point, Vec<Point>);

fn main() {
    println!("Implement me!");
}
