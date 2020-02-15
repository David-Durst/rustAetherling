fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    println!("{}", r1);

    change(s);

}

fn change(mut some_string: String) {
    some_string.push_str(", world");
}
