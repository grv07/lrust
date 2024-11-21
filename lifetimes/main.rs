fn main() {
    // Three rule of rust
    // Rule1:
    fn a(name: &str) -> &str {
        name
    } // =Will be compiles as=>> fn a<'a>(name: &'a str) -> &'a str {}
      // Rule2:
    fn b<'a>(name: &'a str, l_name: &'a str) -> &'a str {
        name
    } // =Will be compiles as=>> fn a<'a, 'b>(name: &'a str, l_name: &'b str) -> &'a str {}

    println!("Hello World!");

    println!("{}", a("Hello"));
    println!("{}", b("Hello", "There"));
}

fn choose<'a>(x: &'a str, y: &'static str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn static_reference() -> &'static str {
    // let message = String::from("Hello, world!");
    // message.as_str() // <- What happens here?

    "just to run"
}

struct TwoRefs<'a, 'b> {
    ref1: &'a str,
    ref2: &'b str,
}

impl<'a, 'b> TwoRefs<'a, 'b> {
    fn swap(&self) -> (&'a str, &'b str) {
        todo!()
    }
}

trait Printable {
    fn display(&self) -> &str;
    fn get_display<'a>(item: &'a impl Printable) -> &'a str;
    fn get_display1<'a, T: Printable>(item: &'a T) -> &'a str;
}
