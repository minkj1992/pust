#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mix_diff<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }

    fn mix(self, other: Point<T, U>) -> Point<T, U> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 1, y: 99.9 };
    let p2 = Point { x: "Hello", y: 'c' };

    let diff_type = Point { x: 5, y: 10.4 }.mix_diff(p2);
    println!("{:#?}", diff_type);
    let same_type = Point { x: 5, y: 10.4 }.mix(p1);
    println!("{:#?}", same_type);

    println!(
        "{:#?}",
        Point { x: 5, y: 10.4 }.mix_diff(Point { x: 1, y: 99.9 })
    );
}
