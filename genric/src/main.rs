/* largest_of(list: &[i32]); and largest_of(list: &[char])
-----> largest_of<T>(list: &[T]) */

// Enums with generic
enum COption<T> {
    Some(T),
    None,
}

enum CResult<T, E> {
    Ok(T),
    Err(E),
}

// struct Point<T> { // When both params are bound to  be same type

// When both can be diff types
#[derive(Debug)]
struct Point<T, Y> {
    x: T,
    y: Y,
}

// Func with generics
impl<T: Copy + std::ops::Mul<Y, Output = T>, Y: Copy> Point<T, Y> {
    fn area(&self) -> T {
        self.x * self.y
    }

    fn mixup<P, V>(self, p: Point<P, V>) -> Point<P, Y> {
        Point { x: p.x, y: self.y }
    }
}

fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let point = Point {
        x: "x".to_string(), // T : String
        y: "y",             // Y: &str
    };

    let point = Point {
        x: 4, // T : i32
        y: 5, // Y: i32
    };

    dbg!(point.area());

    print_type_of(&point.y);
    let p = point.mixup(Point { x: "x", y: "y" });

    dbg!(&p);

    let co = COption::Some("data");
}
