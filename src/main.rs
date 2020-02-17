fn main() {
    //let x = dangle();
    let _z = iter_list();
    let d = String::from("hi");

    let nums = vec![1,5,-3,32];
    println!("max is {}", largest(&nums));
    let v = vec![100, 32, 57];
    for mut i in v {
        i = 2;
    };
    let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
    aetherling::run()
}
/*
fn dangle() -> String {
    let s = String::from("hi");
    s
}
*/

fn iter_list() -> i32 {
    let v = vec![1,3,5,9];

    let mut sum = 0;

    for el in v {
       sum += el
    };

    sum
}
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &'b str {
    y
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}