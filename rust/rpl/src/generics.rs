
fn largest<T>(list: &[T]) -> &T
    where T: PartialOrd
{
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }

    largest
}

pub fn generics_main() -> () {
    let numbers = vec![34, 50, 25, 100, 65];

    let result = largest(&numbers);
    println!("The largest number is {}", result);

    let chars = vec!['y', 'm', 'a', 'q'];

    let result = largest(&chars);
    println!("The largest char is {}", result);

    let p1 = Point{ x: 5, y: 10.4 };
    let p2 = Point{ x: "Hello", y: "World"};
    p2.able();
    let p3 = p1.mixup(p2);
    println!("{:?}", p3);

    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

#[derive(Debug)]
struct Point<T,U> {
    x: T,
    y: U,
}

impl<T,U> Point<T,U> {
    fn mixup<V,W>(self, other: Point<V,W>) -> Point<T,W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

trait Pointable {
    fn able(&self)->();
}

impl<T,U> Pointable for Point<T,U> {
    fn able(&self){
        println!("Able!");
    }
}

enum CustomOption<T> {
    Some(T),
    None,
}

enum CustomResult<T,E> {
    Ok(T),
    Err(E),
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

