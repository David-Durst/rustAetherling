pub mod languages;

pub fn run() {
    let x = ImportantExcerpt {
        part: "hi"
    };
    println!("x's level {}", x.level());
    let s_tmp = String::from("hi");
    println!("s_tmp is {}", str_slice(&s_tmp));

    let v = vec![1,2,3];
    for el in &v {
        println!("el is {}", el)
    }
    for el in &v {
        println!("el is {}", el)
    }

    let c = CustomSmartPointer { data: String::from("my stuff") };
    let d = CustomSmartPointer { data: String::from("other stuff") };
    println!("CustomSmartPointers created.");
}
struct ImportantExcerpt<'a> {
    part: &'a str,
}
impl ImportantExcerpt<'_> {
    fn level(&self) -> i32 {
        3
    }
}

fn str_slice(s: &str) -> &str {
    s
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}