// Option<T>
struct Point<T> {
  x: T,
  y: T
}

struct Line<T> {
  start: Point<T>,
  end: Point<T>
}

fn generics() {
  // let a:Point<u16,i32> = Point { x: 0, y: 4 };
  // let b:Point<f64,f64> = Point { x: 1.2, y: 3.4 };
  let a:Point<f64> = Point { x: 0.0, y: 4f64 };
  let b = Point { x: 1.2, y: 3.4 };

  let myline = Line { start: a, end: b };
}

fn main() {
  generics();
}
